use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::ScientexClient;
use crate::config::Config;
use crate::output::{print_paginated_items, print_result, OutputFormat};

#[derive(Args)]
pub struct ProjectsArgs {
    #[command(subcommand)]
    pub command: ProjectsCommand,
}

#[derive(Subcommand)]
pub enum ProjectsCommand {
    /// List projects.
    List {
        #[arg(short, long, default_value_t = 0)]
        skip: u32,
        #[arg(short, long, default_value_t = 100)]
        limit: u32,
    },
    /// Show project details.
    Get { id: String },
    /// Create a project from a JSON object.
    Create { data: String },
    /// Update a project with a JSON object.
    Update { id: String, data: String },
    /// List project members.
    Members { id: String },
    /// Add a project member.
    AddMember {
        project_id: String,
        user_id: String,
        #[arg(default_value = "member")]
        role: String,
    },
    /// Remove a project member.
    RemoveMember { project_id: String, user_id: String },
}

pub async fn run(
    args: &ProjectsArgs,
    config: &Arc<Config>,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    let client = ScientexClient::new(Arc::clone(config))?;

    match &args.command {
        ProjectsCommand::List { skip, limit } => {
            let projects = client.list_projects(*skip, *limit).await?;
            match format {
                OutputFormat::Json => print_result(&projects, format),
                OutputFormat::Text => print_paginated_items(&projects),
            }
        }
        ProjectsCommand::Get { id } => {
            let project = client.get_project(id).await?;
            print_result(&project, format);
        }
        ProjectsCommand::Create { data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let project = client.create_project(&data).await?;
            print_result(&project, format);
        }
        ProjectsCommand::Update { id, data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let project = client.update_project(id, &data).await?;
            print_result(&project, format);
        }
        ProjectsCommand::Members { id } => {
            let members = client.list_project_members(id).await?;
            match format {
                OutputFormat::Json => print_result(&members, format),
                OutputFormat::Text => print_paginated_items(&members),
            }
        }
        ProjectsCommand::AddMember {
            project_id,
            user_id,
            role,
        } => {
            let member = client.add_project_member(project_id, user_id, role).await?;
            print_result(&member, format);
        }
        ProjectsCommand::RemoveMember {
            project_id,
            user_id,
        } => {
            let result = client.remove_project_member(project_id, user_id).await?;
            print_result(&result, format);
        }
    }
    Ok(())
}
