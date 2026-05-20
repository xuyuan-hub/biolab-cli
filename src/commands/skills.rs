use std::path::{Path, PathBuf};

use clap::{Args, Subcommand, ValueEnum};
use serde::Serialize;

use crate::output::{print_result, OutputFormat};

const SKILL_NAME: &str = "biolab-api";
const SKILL_MD: &str = include_str!("../../skills/biolab-api/SKILL.md");
const STAMP_FILE: &str = ".biolab-cli-version";
const REFERENCES: &[(&str, &str)] = &[
    (
        "orders.md",
        include_str!("../../skills/biolab-api/references/orders.md"),
    ),
    (
        "inventory.md",
        include_str!("../../skills/biolab-api/references/inventory.md"),
    ),
    (
        "templates.md",
        include_str!("../../skills/biolab-api/references/templates.md"),
    ),
    (
        "lab.md",
        include_str!("../../skills/biolab-api/references/lab.md"),
    ),
    (
        "users.md",
        include_str!("../../skills/biolab-api/references/users.md"),
    ),
];

#[derive(Args)]
pub struct SkillsArgs {
    #[command(subcommand)]
    pub command: SkillsCommand,
}

#[derive(Subcommand)]
pub enum SkillsCommand {
    /// Install bundled AI agent skill files
    Install {
        /// Install into the current project or the user's home agent config
        #[arg(long, value_enum, default_value_t = SkillScope::Local)]
        scope: SkillScope,
        /// Agent skills layout to target
        #[arg(long, value_enum, default_value_t = AgentTarget::All)]
        agent: AgentTarget,
        /// Custom skills root. The skill is installed under <path>/biolab-api
        #[arg(long)]
        path: Option<PathBuf>,
        /// Overwrite existing skill files
        #[arg(long)]
        force: bool,
    },
    /// Check whether bundled skills are installed and in sync
    Check {
        /// Check the current project or the user's home agent config
        #[arg(long, value_enum, default_value_t = SkillScope::Local)]
        scope: SkillScope,
        /// Agent skills layout to check
        #[arg(long, value_enum, default_value_t = AgentTarget::All)]
        agent: AgentTarget,
        /// Custom skills root. The skill is expected under <path>/biolab-api
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Print target skill directories
    Path {
        /// Show paths for the current project or the user's home agent config
        #[arg(long, value_enum, default_value_t = SkillScope::Local)]
        scope: SkillScope,
        /// Agent skills layout to show
        #[arg(long, value_enum, default_value_t = AgentTarget::All)]
        agent: AgentTarget,
        /// Custom skills root. The skill path is <path>/biolab-api
        #[arg(long)]
        path: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum SkillScope {
    Local,
    Global,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum AgentTarget {
    All,
    Claude,
    Codex,
}

#[derive(Debug, Serialize)]
struct SkillReport {
    agent: &'static str,
    path: String,
    installed: bool,
    current_version: String,
    target_version: String,
    in_sync: bool,
    action: String,
}

pub fn run(args: &SkillsArgs, format: &OutputFormat) -> anyhow::Result<()> {
    match &args.command {
        SkillsCommand::Install {
            scope,
            agent,
            path,
            force,
        } => {
            let reports = install(*scope, *agent, path.as_deref(), *force)?;
            print_reports(&reports, format);
        }
        SkillsCommand::Check { scope, agent, path } => {
            let reports = check(*scope, *agent, path.as_deref())?;
            print_reports(&reports, format);
        }
        SkillsCommand::Path { scope, agent, path } => {
            let reports = target_paths(*scope, *agent, path.as_deref())?
                .into_iter()
                .map(|target| SkillReport {
                    agent: target.agent,
                    path: target.skill_dir.display().to_string(),
                    installed: skill_files_installed(&target.skill_dir),
                    current_version: read_stamp(&target.skill_dir).unwrap_or_default(),
                    target_version: cli_version().to_string(),
                    in_sync: skill_files_installed(&target.skill_dir)
                        && read_stamp(&target.skill_dir).unwrap_or_default() == cli_version(),
                    action: "path".to_string(),
                })
                .collect::<Vec<_>>();
            print_reports(&reports, format);
        }
    }
    Ok(())
}

fn install(
    scope: SkillScope,
    agent: AgentTarget,
    custom_root: Option<&Path>,
    force: bool,
) -> anyhow::Result<Vec<SkillReport>> {
    let mut reports = Vec::new();
    for target in target_paths(scope, agent, custom_root)? {
        let existing_stamp = read_stamp(&target.skill_dir).unwrap_or_default();
        let already_current =
            skill_files_installed(&target.skill_dir) && existing_stamp == cli_version();
        let action = if already_current && !force {
            "already_in_sync"
        } else {
            std::fs::create_dir_all(&target.skill_dir)?;
            if target.skill_file.exists() {
                "updated"
            } else {
                "installed"
            }
        };

        if action != "already_in_sync" {
            write_skill_files(&target.skill_dir)?;
            std::fs::write(target.skill_dir.join(STAMP_FILE), cli_version())?;
        }

        let current_version = read_stamp(&target.skill_dir).unwrap_or_default();
        reports.push(SkillReport {
            agent: target.agent,
            path: target.skill_dir.display().to_string(),
            installed: skill_files_installed(&target.skill_dir),
            in_sync: current_version == cli_version(),
            current_version,
            target_version: cli_version().to_string(),
            action: action.to_string(),
        });
    }
    Ok(reports)
}

fn check(
    scope: SkillScope,
    agent: AgentTarget,
    custom_root: Option<&Path>,
) -> anyhow::Result<Vec<SkillReport>> {
    let mut reports = Vec::new();
    for target in target_paths(scope, agent, custom_root)? {
        let current_version = read_stamp(&target.skill_dir).unwrap_or_default();
        let installed = skill_files_installed(&target.skill_dir);
        reports.push(SkillReport {
            agent: target.agent,
            path: target.skill_dir.display().to_string(),
            installed,
            in_sync: installed && current_version == cli_version(),
            current_version,
            target_version: cli_version().to_string(),
            action: "checked".to_string(),
        });
    }
    Ok(reports)
}

#[derive(Debug)]
struct SkillTarget {
    agent: &'static str,
    skill_dir: PathBuf,
    skill_file: PathBuf,
}

fn target_paths(
    scope: SkillScope,
    agent: AgentTarget,
    custom_root: Option<&Path>,
) -> anyhow::Result<Vec<SkillTarget>> {
    let roots = if let Some(root) = custom_root {
        vec![("custom", root.to_path_buf())]
    } else {
        match agent {
            AgentTarget::All => vec![
                ("claude", default_root(scope, "claude")?),
                ("codex", default_root(scope, "codex")?),
            ],
            AgentTarget::Claude => vec![("claude", default_root(scope, "claude")?)],
            AgentTarget::Codex => vec![("codex", default_root(scope, "codex")?)],
        }
    };

    Ok(roots
        .into_iter()
        .map(|(agent, root)| {
            let skill_dir = root.join(SKILL_NAME);
            let skill_file = skill_dir.join("SKILL.md");
            SkillTarget {
                agent,
                skill_dir,
                skill_file,
            }
        })
        .collect())
}

fn default_root(scope: SkillScope, agent: &str) -> anyhow::Result<PathBuf> {
    let root = match scope {
        SkillScope::Local => std::env::current_dir()?,
        SkillScope::Global => dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
    };

    let path = match agent {
        "claude" => root.join(".claude").join("skills"),
        "codex" => root.join(".codex").join("skills"),
        _ => root,
    };
    Ok(path)
}

fn read_stamp(skill_dir: &Path) -> Option<String> {
    std::fs::read_to_string(skill_dir.join(STAMP_FILE))
        .ok()
        .map(|value| value.trim().to_string())
}

fn write_skill_files(skill_dir: &Path) -> anyhow::Result<()> {
    std::fs::write(skill_dir.join("SKILL.md"), SKILL_MD)?;

    let references_dir = skill_dir.join("references");
    std::fs::create_dir_all(&references_dir)?;
    for (file_name, content) in REFERENCES {
        std::fs::write(references_dir.join(file_name), content)?;
    }
    Ok(())
}

fn skill_files_installed(skill_dir: &Path) -> bool {
    skill_dir.join("SKILL.md").exists()
        && REFERENCES
            .iter()
            .all(|(file_name, _)| skill_dir.join("references").join(file_name).exists())
}

fn print_reports(reports: &[SkillReport], format: &OutputFormat) {
    match format {
        OutputFormat::Json => print_result(&reports, format),
        OutputFormat::Text => {
            for report in reports {
                let status = if report.in_sync {
                    "in sync"
                } else {
                    "needs install"
                };
                println!(
                    "{}  {}  {}  {}",
                    report.agent, report.action, status, report.path
                );
            }
        }
    }
}

fn cli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
