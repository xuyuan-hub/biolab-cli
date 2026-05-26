use std::sync::Arc;

use biolab::commands::{inventory, lab, orders, projects, skills, templates, update, users};
use biolab::config::Config;
use biolab::output::OutputFormat;
use biolab::{check_status, login, logout, poll_login_from_env, LoginMode};
use clap::{Args, Parser, Subcommand, ValueEnum};
use colored::Colorize;

/// Biolab lab management CLI.
#[derive(Parser)]
#[command(name = "biolab", version, about, long_about = None)]
struct Cli {
    /// Output format.
    #[arg(short, long, value_enum, default_value_t = OutputFormatArg::Text, global = true)]
    format: OutputFormatArg,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, ValueEnum)]
enum OutputFormatArg {
    Text,
    Json,
}

impl From<&OutputFormatArg> for OutputFormat {
    fn from(val: &OutputFormatArg) -> Self {
        match val {
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Feishu OAuth login.
    Login(LoginArgs),
    /// Finish background login polling.
    #[command(hide = true)]
    LoginPoll,
    /// Log out and remove the local token.
    Logout,
    /// Check login status.
    Status,

    /// Current user management.
    Me(users::MeArgs),

    /// Order management.
    Orders(orders::OrdersArgs),

    /// Template management.
    Templates(templates::TemplatesArgs),

    /// Inventory management.
    Inventory(inventory::InventoryArgs),

    /// Lab management.
    Lab(lab::LabArgs),

    /// Project management.
    Projects(projects::ProjectsArgs),

    /// AI agent skill installation and checks.
    Skills(skills::SkillsArgs),

    /// Check CLI updates.
    Update(update::UpdateArgs),
}

#[derive(Args)]
struct LoginArgs {
    /// Run login polling in the background for agent workflows.
    #[arg(long, alias = "no-wait")]
    background: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = Arc::new(Config::new());
    let format = OutputFormat::from(&cli.format);

    let result = match cli.command {
        None => {
            println!("{}", "Biolab CLI".bold());
            println!("\nRun biolab --help to see available commands.\n");
            return;
        }
        Some(Commands::Login(args)) => {
            let mode = if args.background {
                LoginMode::Background
            } else {
                LoginMode::Wait
            };
            login(&config, mode).await;
            Ok(())
        }
        Some(Commands::LoginPoll) => {
            if !poll_login_from_env(&config).await {
                std::process::exit(1);
            }
            Ok(())
        }
        Some(Commands::Logout) => {
            logout(&config);
            Ok(())
        }
        Some(Commands::Status) => {
            let logged_in = check_status(&config);
            if !logged_in {
                std::process::exit(1);
            }
            Ok(())
        }
        Some(Commands::Me(args)) => users::run(&args, &config, &format).await,
        Some(Commands::Orders(args)) => orders::run(&args, &config, &format).await,
        Some(Commands::Templates(args)) => templates::run(&args, &config, &format).await,
        Some(Commands::Inventory(args)) => inventory::run(&args, &config, &format).await,
        Some(Commands::Lab(args)) => lab::run(&args, &config, &format).await,
        Some(Commands::Projects(args)) => projects::run(&args, &config, &format).await,
        Some(Commands::Skills(args)) => skills::run(&args, &format),
        Some(Commands::Update(args)) => update::run(&args, &format).await,
    };

    if let Err(e) = result {
        eprintln!("{}: {e}", "Error".red().bold());
        std::process::exit(1);
    }
}
