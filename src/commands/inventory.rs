use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{print_result, print_stocks, OutputFormat};

#[derive(Args)]
pub struct InventoryArgs {
    #[command(subcommand)]
    pub command: InventoryCommand,
}

#[derive(Subcommand)]
pub enum InventoryCommand {
    /// 库存列表
    List {
        #[arg(long)]
        primer_name: Option<String>,
        #[arg(long)]
        location_id: Option<String>,
        #[arg(long)]
        low_stock: bool,
    },
    /// 库存详情（含交易记录）
    Get { id: String },
    /// 库存统计
    Stats,
    /// 入库
    Checkin {
        id: String,
        #[arg(long)]
        quantity: f64,
        #[arg(long, default_value = "")]
        purpose: String,
    },
    /// 出库
    Checkout {
        id: String,
        #[arg(long)]
        quantity: f64,
        #[arg(long, default_value = "")]
        purpose: String,
        #[arg(long, default_value = "")]
        experiment_ref: String,
    },
    /// 存储位置列表
    Locations,
    /// 创建存储位置
    CreateLocation {
        name: String,
        #[arg(long)]
        parent_id: Option<String>,
    },
}

pub async fn run(
    args: &InventoryArgs,
    config: &Arc<Config>,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    let client = BiolabClient::new(Arc::clone(config))?;

    match &args.command {
        InventoryCommand::List {
            primer_name,
            location_id,
            low_stock,
        } => {
            let stocks = client
                .list_stocks(primer_name.as_deref(), location_id.as_deref(), *low_stock)
                .await?;
            match format {
                OutputFormat::Json => print_result(&stocks, format),
                OutputFormat::Text => print_stocks(&stocks),
            }
        }
        InventoryCommand::Get { id } => {
            let stock = client.get_stock(id).await?;
            print_result(&stock, format);
        }
        InventoryCommand::Stats => {
            let stats = client.get_stock_stats().await?;
            print_result(&stats, format);
        }
        InventoryCommand::Checkin {
            id,
            quantity,
            purpose,
        } => {
            let stock = client.checkin(id, *quantity, purpose).await?;
            print_result(&stock, format);
        }
        InventoryCommand::Checkout {
            id,
            quantity,
            purpose,
            experiment_ref,
        } => {
            let stock = client
                .checkout(id, *quantity, purpose, experiment_ref)
                .await?;
            print_result(&stock, format);
        }
        InventoryCommand::Locations => {
            let locations = client.list_locations().await?;
            print_result(&locations, format);
        }
        InventoryCommand::CreateLocation { name, parent_id } => {
            let location = client.create_location(name, parent_id.as_deref()).await?;
            print_result(&location, format);
        }
    }
    Ok(())
}
