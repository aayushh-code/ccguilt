use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ClaudeDataDir {
    pub base: PathBuf,
}

impl ClaudeDataDir {
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }

    pub fn default_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".claude"))
    }

    pub fn projects_dir(&self) -> PathBuf {
        self.base.join("projects")
    }

    pub fn stats_cache_path(&self) -> PathBuf {
        self.base.join("stats-cache.json")
    }

    /// Returns all JSONL session files, optionally filtered by project substring
    pub fn jsonl_files(&self, project_filter: Option<&str>) -> Vec<PathBuf> {
        let projects_dir = self.projects_dir();
        if !projects_dir.exists() {
            return Vec::new();
        }

        let mut files = Vec::new();

        for entry in WalkDir::new(&projects_dir)
            .min_depth(1)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "jsonl") {
                if let Some(filter) = project_filter {
                    let project_name = Self::project_name_from_jsonl(path, &projects_dir);
                    let decoded = decode_project_name(&project_name);
                    if !project_name.contains(filter) && !decoded.contains(filter) {
                        continue;
                    }
                }
                files.push(path.to_path_buf());
            }
        }

        files
    }

    /// Extract project directory name from a JSONL file path
    fn project_name_from_jsonl(jsonl_path: &Path, projects_dir: &Path) -> String {
        if let Ok(relative) = jsonl_path.strip_prefix(projects_dir) {
            if let Some(first_component) = relative.components().next() {
                return first_component.as_os_str().to_string_lossy().to_string();
            }
        }
        String::new()
    }

    /// Count the number of project directories
    pub fn project_count(&self) -> usize {
        let projects_dir = self.projects_dir();
        if !projects_dir.exists() {
            return 0;
        }
        std::fs::read_dir(projects_dir)
            .map(|entries| entries.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()).count())
            .unwrap_or(0)
    }
}

/// Decode Claude Code's project directory naming convention
/// "-home-it8-Repos" → "/home/it8/Repos"
pub fn decode_project_name(encoded: &str) -> String {
    if encoded.is_empty() {
        return String::new();
    }
    // The encoding replaces / with - and prepends a -
    // So "-home-it8-Repos" comes from "/home/it8/Repos"
    // This is ambiguous if path components contain dashes, but it's the best we can do
    let mut result = String::new();
    let chars: Vec<char> = encoded.chars().collect();
    for ch in &chars {
        if *ch == '-' {
            result.push('/');
        } else {
            result.push(*ch);
        }
    }
    result
}
