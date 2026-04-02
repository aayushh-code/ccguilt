use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

use crate::models::{ModelTier, TokenRecord};

const SCHEMA_VERSION: i64 = 1;

/// Metadata about a JSONL file on disk.
struct FileInfo {
    path: PathBuf,
    mtime_secs: i64,
    file_size: i64,
}

/// Metadata about a JSONL file as recorded in the DB.
struct IngestedFileInfo {
    mtime_secs: i64,
    file_size: i64,
}

fn open_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database at {}", db_path.display()))?;
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA synchronous=NORMAL;")?;
    Ok(conn)
}

fn ensure_schema(conn: &Connection) -> Result<()> {
    // Check if schema_version table exists
    let has_schema: bool = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='schema_version'")?
        .exists([])?;

    if !has_schema {
        create_schema_v1(conn)?;
        return Ok(());
    }

    let version: i64 =
        conn.query_row("SELECT version FROM schema_version", [], |row| row.get(0))?;

    if version < SCHEMA_VERSION {
        // Future migrations would go here
        conn.execute(
            "UPDATE schema_version SET version = ?1",
            params![SCHEMA_VERSION],
        )?;
    }

    Ok(())
}

fn create_schema_v1(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE schema_version (version INTEGER NOT NULL);
        INSERT INTO schema_version (version) VALUES (1);

        CREATE TABLE ingested_files (
            file_path   TEXT PRIMARY KEY,
            mtime_secs  INTEGER NOT NULL,
            file_size   INTEGER NOT NULL,
            ingested_at TEXT NOT NULL
        );

        CREATE TABLE token_records (
            id                          INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp                   TEXT NOT NULL,
            session_id                  TEXT NOT NULL,
            project_name                TEXT NOT NULL,
            model                       TEXT NOT NULL,
            input_tokens                INTEGER NOT NULL,
            output_tokens               INTEGER NOT NULL,
            cache_creation_input_tokens INTEGER NOT NULL,
            cache_read_input_tokens     INTEGER NOT NULL,
            source_file                 TEXT NOT NULL
        );

        CREATE INDEX idx_records_timestamp ON token_records(timestamp);
        CREATE INDEX idx_records_session ON token_records(session_id);
        CREATE INDEX idx_records_project ON token_records(project_name);
        CREATE INDEX idx_records_source ON token_records(source_file);
        ",
    )?;
    Ok(())
}

/// Stat JSONL files on disk to get mtime and size.
fn stat_files(paths: &[PathBuf]) -> Vec<FileInfo> {
    paths
        .iter()
        .filter_map(|p| {
            let meta = std::fs::metadata(p).ok()?;
            use std::os::unix::fs::MetadataExt;
            Some(FileInfo {
                path: p.clone(),
                mtime_secs: meta.mtime(),
                file_size: meta.size() as i64,
            })
        })
        .collect()
}

/// Determine which files need (re-)ingestion and which DB entries are orphaned.
fn classify_files(conn: &Connection, disk_files: &[FileInfo]) -> Result<Vec<usize>> {
    // Load all ingested file info from DB
    let mut stmt = conn.prepare("SELECT file_path, mtime_secs, file_size FROM ingested_files")?;
    let db_files: std::collections::HashMap<String, IngestedFileInfo> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                IngestedFileInfo {
                    mtime_secs: row.get(1)?,
                    file_size: row.get(2)?,
                },
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    // Find files that need ingestion (new or changed)
    let mut need_ingestion: Vec<usize> = Vec::new();

    for (i, fi) in disk_files.iter().enumerate() {
        let path_str = fi.path.to_string_lossy().to_string();

        match db_files.get(&path_str) {
            None => need_ingestion.push(i),
            Some(db_info) => {
                if db_info.mtime_secs != fi.mtime_secs || db_info.file_size != fi.file_size {
                    need_ingestion.push(i);
                }
            }
        }
    }

    Ok(need_ingestion)
}

/// Insert token records for a single file into the database.
fn insert_records(
    conn: &Connection,
    source_file: &str,
    records: &[TokenRecord],
    mtime_secs: i64,
    file_size: i64,
) -> Result<()> {
    let mut stmt = conn.prepare_cached(
        "INSERT INTO token_records (
            timestamp, session_id, project_name, model,
            input_tokens, output_tokens,
            cache_creation_input_tokens, cache_read_input_tokens,
            source_file
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    for r in records {
        stmt.execute(params![
            r.timestamp.to_rfc3339(),
            r.session_id,
            r.project_name,
            r.model.as_db_str(),
            r.input_tokens,
            r.output_tokens,
            r.cache_creation_input_tokens,
            r.cache_read_input_tokens,
            source_file,
        ])?;
    }

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR REPLACE INTO ingested_files (file_path, mtime_secs, file_size, ingested_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![source_file, mtime_secs, file_size, now],
    )?;

    Ok(())
}

/// Query all token records, optionally filtered by date range and/or project.
fn query_records(
    conn: &Connection,
    since: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
    project_filter: Option<&str>,
) -> Result<Vec<TokenRecord>> {
    let mut sql = String::from(
        "SELECT timestamp, session_id, project_name, model,
                input_tokens, output_tokens,
                cache_creation_input_tokens, cache_read_input_tokens
         FROM token_records WHERE 1=1",
    );
    let mut param_values: Vec<String> = Vec::new();

    if let Some(s) = since {
        param_values.push(s.to_rfc3339());
        sql.push_str(&format!(" AND timestamp >= ?{}", param_values.len()));
    }
    if let Some(u) = until {
        param_values.push(u.to_rfc3339());
        sql.push_str(&format!(" AND timestamp <= ?{}", param_values.len()));
    }
    if let Some(pf) = project_filter {
        param_values.push(format!("%{}%", pf));
        sql.push_str(&format!(" AND project_name LIKE ?{}", param_values.len()));
    }

    sql.push_str(" ORDER BY timestamp");

    let mut stmt = conn.prepare(&sql)?;
    let params: Vec<&dyn rusqlite::types::ToSql> = param_values
        .iter()
        .map(|v| v as &dyn rusqlite::types::ToSql)
        .collect();

    let records = stmt
        .query_map(params.as_slice(), |row| {
            let ts_str: String = row.get(0)?;
            let timestamp = ts_str
                .parse::<DateTime<Utc>>()
                .unwrap_or_else(|_| Utc::now());
            Ok(TokenRecord {
                timestamp,
                session_id: row.get(1)?,
                project_name: row.get(2)?,
                model: ModelTier::from_db_str(&row.get::<_, String>(3)?),
                input_tokens: row.get(4)?,
                output_tokens: row.get(5)?,
                cache_creation_input_tokens: row.get(6)?,
                cache_read_input_tokens: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(records)
}

/// Main entry point: incrementally ingest new/changed files, then query.
pub fn load_records(
    db_path: &Path,
    jsonl_files: &[PathBuf],
    since: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
    project_filter: Option<&str>,
    rebuild: bool,
    quiet: bool,
) -> Result<Vec<TokenRecord>> {
    // If rebuild requested, delete existing DB
    if rebuild && db_path.exists() {
        std::fs::remove_file(db_path)?;
    }

    let conn = open_db(db_path)?;
    ensure_schema(&conn)?;

    // Stat all JSONL files on disk
    let disk_files = stat_files(jsonl_files);

    // Classify files: which need ingestion (new or changed)
    let need_ingestion_indices = classify_files(&conn, &disk_files)?;

    if !need_ingestion_indices.is_empty() {
        if !quiet {
            use colored::Colorize;
            eprintln!(
                "  {} Ingesting {} new/changed files into SQLite cache...",
                ">>".green().bold(),
                need_ingestion_indices.len()
            );
        }

        // Collect files that need parsing
        let files_to_parse: Vec<&FileInfo> = need_ingestion_indices
            .iter()
            .map(|&i| &disk_files[i])
            .collect();

        // Parse in parallel (no date filters — store everything)
        let parsed: Vec<(&FileInfo, Vec<TokenRecord>)> = files_to_parse
            .par_iter()
            .filter_map(|fi| {
                let records =
                    crate::data::jsonl::parse_single_file(&fi.path, None, None, None).ok()?;
                Some((*fi, records))
            })
            .collect();

        // Serial insert in one transaction
        conn.execute_batch("BEGIN")?;

        for (fi, records) in &parsed {
            let path_str = fi.path.to_string_lossy().to_string();
            // Purge old records for this file if it was stale
            conn.execute(
                "DELETE FROM token_records WHERE source_file = ?1",
                params![path_str],
            )?;
            conn.execute(
                "DELETE FROM ingested_files WHERE file_path = ?1",
                params![path_str],
            )?;
            insert_records(&conn, &path_str, records, fi.mtime_secs, fi.file_size)?;
        }

        conn.execute_batch("COMMIT")?;
    }

    // Query with date/project filters pushed to SQL
    query_records(&conn, since, until, project_filter)
}
