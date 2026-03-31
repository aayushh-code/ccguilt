use anyhow::{bail, Result};
use clap::CommandFactory;
use clap_complete::Shell;
use colored::Colorize;
use std::path::PathBuf;

use crate::cli::Args;

fn detect_shell() -> Result<Shell> {
    let shell_env = std::env::var("SHELL").unwrap_or_default();
    let name = shell_env
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_lowercase();

    match name.as_str() {
        "bash" => Ok(Shell::Bash),
        "zsh" => Ok(Shell::Zsh),
        "fish" => Ok(Shell::Fish),
        "elvish" => Ok(Shell::Elvish),
        "powershell" | "pwsh" => Ok(Shell::PowerShell),
        _ => bail!(
            "Could not detect shell from $SHELL={:?}. Pass the shell explicitly:\n  \
             ccguilt --setup-completions bash",
            shell_env
        ),
    }
}

fn completion_install_path(shell: Shell) -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;

    match shell {
        Shell::Bash => Ok(home.join(".local/share/bash-completion/completions/ccguilt")),
        Shell::Zsh => Ok(home.join(".zfunc/_ccguilt")),
        Shell::Fish => Ok(home.join(".config/fish/completions/ccguilt.fish")),
        Shell::Elvish => Ok(home.join(".config/elvish/lib/completions/ccguilt.elv")),
        Shell::PowerShell => Ok(home.join(".config/powershell/completions/ccguilt.ps1")),
        _ => bail!("Unsupported shell for auto-install"),
    }
}

fn generate_completion_script(shell: Shell) -> Vec<u8> {
    let mut buf = Vec::new();
    clap_complete::generate(shell, &mut Args::command(), "ccguilt", &mut buf);
    buf
}

pub fn setup_completions(shell_arg: &str) -> Result<()> {
    let shell = if shell_arg == "auto" {
        let detected = detect_shell()?;
        eprintln!(
            "  {} Detected shell: {}",
            ">>".bold(),
            format!("{detected:?}").to_lowercase().cyan()
        );
        detected
    } else {
        shell_arg
            .parse::<Shell>()
            .map_err(|_| anyhow::anyhow!("Unknown shell: {shell_arg}. Try: bash, zsh, fish"))?
    };

    let path = completion_install_path(shell)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let script = generate_completion_script(shell);
    std::fs::write(&path, script)?;

    let display_path = path.display();
    eprintln!(
        "  {} Completions written to {}",
        ">>".bold(),
        display_path.to_string().green()
    );

    match shell {
        Shell::Bash => {
            eprintln!(
                "  {} Restart your shell or run: {}",
                ">>".bold(),
                format!("source {display_path}").yellow()
            );
        }
        Shell::Zsh => {
            eprintln!(
                "  {} Add this to your {} if not already present:",
                ">>".bold(),
                "~/.zshrc".cyan()
            );
            eprintln!(
                "     {}",
                "fpath=(~/.zfunc $fpath); autoload -Uz compinit && compinit".yellow()
            );
            eprintln!("  {} Then restart your shell.", ">>".bold());
        }
        Shell::Fish => {
            eprintln!(
                "  {} Fish loads this automatically. Restart your shell.",
                ">>".bold()
            );
        }
        _ => {
            eprintln!("  {} Restart your shell to enable completions.", ">>".bold());
        }
    }

    Ok(())
}
