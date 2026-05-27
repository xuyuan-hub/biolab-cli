use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{print_result, OutputFormat};

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
            print_result(&records, format);
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
            print_result(&orders, format);
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
