use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{print_paginated_items, print_result, OutputFormat};

#[derive(Args)]
pub struct ProjectArgs {
    /// Project slug, for example `tashan`.
    pub slug: String,
    #[command(subcommand)]
    pub command: ProjectCommand,
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Show project information by slug.
    Info,
    /// Project germplasm workflows.
    Germplasm {
        #[command(subcommand)]
        command: GermplasmCommand,
    },
    /// Project planting workflows.
    Planting {
        #[command(subcommand)]
        command: PlantingCommand,
    },
}

#[derive(Subcommand)]
pub enum GermplasmCommand {
    /// List germplasm records.
    List {
        #[arg(short, long, default_value_t = 0)]
        skip: u32,
        #[arg(short, long, default_value_t = 10)]
        limit: u32,
        #[arg(long)]
        search: Option<String>,
        #[arg(long)]
        filters: Option<String>,
    },
    /// Show one germplasm record.
    Get { id: String },
    /// Create a germplasm record from a JSON object.
    Create { data: String },
    /// Update a germplasm record with a JSON object.
    Update { id: String, data: String },
    /// Delete a germplasm record.
    Delete { id: String },
    /// List sequencing files attached to a germplasm record.
    SequencingFiles { id: String },
    /// List stocks for a germplasm record.
    Stocks { id: String },
}

#[derive(Subcommand)]
pub enum PlantingCommand {
    /// List planting orders.
    List {
        #[arg(short, long, default_value_t = 0)]
        skip: u32,
        #[arg(short, long, default_value_t = 100)]
        limit: u32,
    },
    /// Show one planting order.
    Get { id: String },
    /// Create a planting order from a JSON object.
    Create { data: String },
    /// Update a planting order with a JSON object.
    Update { id: String, data: String },
    /// List planting order items.
    Items { id: String },
    /// List harvest records for a planting order.
    Harvests { id: String },
    /// Create harvest records for a planting order from a JSON object.
    CreateHarvest { id: String, data: String },
}

pub async fn run(
    args: &ProjectArgs,
    config: &Arc<Config>,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    let client = BiolabClient::new(Arc::clone(config))?;

    match &args.command {
        ProjectCommand::Info => {
            let project = client.get_project_by_slug(&args.slug).await?;
            print_result(&project, format);
        }
        ProjectCommand::Germplasm { command } => {
            run_germplasm(&client, &args.slug, command, format).await?
        }
        ProjectCommand::Planting { command } => {
            run_planting(&client, &args.slug, command, format).await?
        }
    }
    Ok(())
}

async fn run_germplasm(
    client: &BiolabClient,
    slug: &str,
    command: &GermplasmCommand,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    match command {
        GermplasmCommand::List {
            skip,
            limit,
            search,
            filters,
        } => {
            let records = client
                .list_project_germplasm(slug, *skip, *limit, search.as_deref(), filters.as_deref())
                .await?;
            match format {
                OutputFormat::Json => print_result(&records, format),
                OutputFormat::Text => print_paginated_items(&records),
            }
        }
        GermplasmCommand::Get { id } => {
            let record = client.get_project_germplasm(slug, id).await?;
            print_result(&record, format);
        }
        GermplasmCommand::Create { data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let record = client.create_project_germplasm(slug, &data).await?;
            print_result(&record, format);
        }
        GermplasmCommand::Update { id, data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let record = client.update_project_germplasm(slug, id, &data).await?;
            print_result(&record, format);
        }
        GermplasmCommand::Delete { id } => {
            let result = client.delete_project_germplasm(slug, id).await?;
            print_result(&result, format);
        }
        GermplasmCommand::SequencingFiles { id } => {
            let files = client
                .list_project_germplasm_sequencing_files(slug, id)
                .await?;
            print_result(&files, format);
        }
        GermplasmCommand::Stocks { id } => {
            let stocks = client.list_project_germplasm_stocks(slug, id).await?;
            print_result(&stocks, format);
        }
    }
    Ok(())
}

async fn run_planting(
    client: &BiolabClient,
    slug: &str,
    command: &PlantingCommand,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    match command {
        PlantingCommand::List { skip, limit } => {
            let orders = client
                .list_project_planting_orders(slug, *skip, *limit)
                .await?;
            match format {
                OutputFormat::Json => print_result(&orders, format),
                OutputFormat::Text => print_paginated_items(&orders),
            }
        }
        PlantingCommand::Get { id } => {
            let order = client.get_project_planting_order(slug, id).await?;
            print_result(&order, format);
        }
        PlantingCommand::Create { data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let order = client.create_project_planting_order(slug, &data).await?;
            print_result(&order, format);
        }
        PlantingCommand::Update { id, data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let order = client
                .update_project_planting_order(slug, id, &data)
                .await?;
            print_result(&order, format);
        }
        PlantingCommand::Items { id } => {
            let items = client.list_project_planting_items(slug, id).await?;
            print_result(&items, format);
        }
        PlantingCommand::Harvests { id } => {
            let harvests = client.list_project_planting_harvests(slug, id).await?;
            print_result(&harvests, format);
        }
        PlantingCommand::CreateHarvest { id, data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let harvests = client
                .create_project_planting_harvest(slug, id, &data)
                .await?;
            print_result(&harvests, format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: TestCommand,
    }

    #[derive(Subcommand)]
    enum TestCommand {
        Project(ProjectArgs),
    }

    fn parse_project(args: &[&str]) -> ProjectArgs {
        let cli = TestCli::try_parse_from(std::iter::once("biolab").chain(args.iter().copied()))
            .expect("project command should parse");
        match cli.command {
            TestCommand::Project(args) => args,
        }
    }

    #[test]
    fn parses_project_info_command() {
        let args = parse_project(&["project", "tashan", "info"]);

        assert_eq!(args.slug, "tashan");
        assert!(matches!(args.command, ProjectCommand::Info));
    }

    #[test]
    fn parses_germplasm_list_options() {
        let args = parse_project(&[
            "project",
            "tashan",
            "germplasm",
            "list",
            "--skip",
            "20",
            "--limit",
            "50",
            "--search",
            "rice A",
            "--filters",
            r#"[{"field":"name","operator":"contains","value":"A"}]"#,
        ]);

        match args.command {
            ProjectCommand::Germplasm {
                command:
                    GermplasmCommand::List {
                        skip,
                        limit,
                        search,
                        filters,
                    },
            } => {
                assert_eq!(skip, 20);
                assert_eq!(limit, 50);
                assert_eq!(search.as_deref(), Some("rice A"));
                assert_eq!(
                    filters.as_deref(),
                    Some(r#"[{"field":"name","operator":"contains","value":"A"}]"#)
                );
            }
            _ => panic!("expected germplasm list command"),
        }
    }

    #[test]
    fn parses_germplasm_list_defaults() {
        let args = parse_project(&["project", "tashan", "germplasm", "list"]);

        match args.command {
            ProjectCommand::Germplasm {
                command:
                    GermplasmCommand::List {
                        skip,
                        limit,
                        search,
                        filters,
                    },
            } => {
                assert_eq!(skip, 0);
                assert_eq!(limit, 10);
                assert_eq!(search, None);
                assert_eq!(filters, None);
            }
            _ => panic!("expected germplasm list command"),
        }
    }

    #[test]
    fn parses_germplasm_mutation_commands() {
        let create = parse_project(&[
            "project",
            "tashan",
            "germplasm",
            "create",
            r#"{"name":"A"}"#,
        ]);
        match create.command {
            ProjectCommand::Germplasm {
                command: GermplasmCommand::Create { data },
            } => assert_eq!(data, r#"{"name":"A"}"#),
            _ => panic!("expected germplasm create command"),
        }

        let update = parse_project(&[
            "project",
            "tashan",
            "germplasm",
            "update",
            "gp-1",
            r#"{"name":"B"}"#,
        ]);
        match update.command {
            ProjectCommand::Germplasm {
                command: GermplasmCommand::Update { id, data },
            } => {
                assert_eq!(id, "gp-1");
                assert_eq!(data, r#"{"name":"B"}"#);
            }
            _ => panic!("expected germplasm update command"),
        }
    }

    #[test]
    fn parses_planting_list_defaults_and_overrides() {
        let defaults = parse_project(&["project", "tashan", "planting", "list"]);
        match defaults.command {
            ProjectCommand::Planting {
                command: PlantingCommand::List { skip, limit },
            } => {
                assert_eq!(skip, 0);
                assert_eq!(limit, 100);
            }
            _ => panic!("expected planting list command"),
        }

        let overrides = parse_project(&[
            "project", "tashan", "planting", "list", "--skip", "10", "--limit", "25",
        ]);
        match overrides.command {
            ProjectCommand::Planting {
                command: PlantingCommand::List { skip, limit },
            } => {
                assert_eq!(skip, 10);
                assert_eq!(limit, 25);
            }
            _ => panic!("expected planting list command"),
        }
    }

    #[test]
    fn parses_planting_subresource_commands() {
        let items = parse_project(&["project", "tashan", "planting", "items", "ord-1"]);
        match items.command {
            ProjectCommand::Planting {
                command: PlantingCommand::Items { id },
            } => assert_eq!(id, "ord-1"),
            _ => panic!("expected planting items command"),
        }

        let create_harvest = parse_project(&[
            "project",
            "tashan",
            "planting",
            "create-harvest",
            "ord-1",
            r#"{"items":[]}"#,
        ]);
        match create_harvest.command {
            ProjectCommand::Planting {
                command: PlantingCommand::CreateHarvest { id, data },
            } => {
                assert_eq!(id, "ord-1");
                assert_eq!(data, r#"{"items":[]}"#);
            }
            _ => panic!("expected planting create-harvest command"),
        }
    }

    #[test]
    fn rejects_unknown_project_subcommand() {
        assert!(TestCli::try_parse_from(["biolab", "project", "tashan", "unknown"]).is_err());
    }
}
