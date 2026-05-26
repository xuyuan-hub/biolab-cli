use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{print_order, print_order_brief, print_result, OutputFormat};

#[derive(Args)]
pub struct OrdersArgs {
    #[command(subcommand)]
    pub command: OrdersCommand,
}

#[derive(Subcommand)]
pub enum OrdersCommand {
    /// Show order statistics.
    Stats,
    /// List orders.
    List {
        #[arg(short, long, default_value_t = 0)]
        skip: u32,
        #[arg(short, long, default_value_t = 100)]
        limit: u32,
    },
    /// List orders waiting for my approval.
    PendingApprovals,
    /// Show order details.
    Get { id: String },
    /// Create a primer synthesis order from a JSON file.
    CreatePrimer { file: String },
    /// Create a sequencing order from a JSON file.
    CreateSequencing { file: String },
    /// Update an order with a JSON object.
    Update { id: String, data: String },
    /// Resend order email for pending orders.
    Resend { id: String },
    /// Send order email.
    Send { id: String },
    /// Approve an order.
    Approve { id: String },
    /// Reject an order.
    Reject { id: String },
    /// Download order Excel.
    Download {
        id: String,
        #[arg(default_value = "order.xlsx")]
        output: String,
    },
    /// Download primer Excel template.
    DownloadPrimerTemplate {
        #[arg(default_value = "primer_template.xlsx")]
        output: String,
    },
    /// Download sequencing Excel template.
    DownloadSequencingTemplate {
        #[arg(default_value = "sequencing_template.xlsx")]
        output: String,
    },
    /// Upload and parse primer Excel.
    UploadPrimerExcel { file: String },
    /// Upload and parse sequencing Excel.
    UploadSequencingExcel { file: String },
}

pub async fn run(
    args: &OrdersArgs,
    config: &Arc<Config>,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    let client = BiolabClient::new(Arc::clone(config))?;

    match &args.command {
        OrdersCommand::Stats => {
            let stats = client.get_order_stats().await?;
            print_result(&stats, format);
        }
        OrdersCommand::List { skip, limit } => {
            let orders = client.list_orders(*skip, *limit).await?;
            if orders.is_empty() {
                println!("No orders");
                return Ok(());
            }
            match format {
                OutputFormat::Json => print_result(&orders, format),
                OutputFormat::Text => {
                    for o in &orders {
                        print_order_brief(o);
                    }
                }
            }
        }
        OrdersCommand::PendingApprovals => {
            let orders = client.list_pending_approvals().await?;
            if orders.is_empty() {
                println!("No pending approvals");
                return Ok(());
            }
            match format {
                OutputFormat::Json => print_result(&orders, format),
                OutputFormat::Text => {
                    for o in &orders {
                        print_order_brief(o);
                    }
                }
            }
        }
        OrdersCommand::Get { id } => {
            let order = client.get_order(id).await?;
            match format {
                OutputFormat::Json => print_result(&order, format),
                OutputFormat::Text => print_order(&order),
            }
        }
        OrdersCommand::CreatePrimer { file } => {
            let content = std::fs::read_to_string(file)?;
            let order: serde_json::Value = serde_json::from_str(&content)?;
            let result = client.create_primer_order(&order).await?;
            match format {
                OutputFormat::Json => print_result(&result, format),
                OutputFormat::Text => print_order(&result),
            }
        }
        OrdersCommand::CreateSequencing { file } => {
            let content = std::fs::read_to_string(file)?;
            let order: serde_json::Value = serde_json::from_str(&content)?;
            let result = client.create_sequencing_order(&order).await?;
            match format {
                OutputFormat::Json => print_result(&result, format),
                OutputFormat::Text => print_order(&result),
            }
        }
        OrdersCommand::Update { id, data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let result = client.update_order(id, &data).await?;
            match format {
                OutputFormat::Json => print_result(&result, format),
                OutputFormat::Text => print_order(&result),
            }
        }
        OrdersCommand::Resend { id } => {
            let result = client.resend_order(id).await?;
            print_result(&result, format);
        }
        OrdersCommand::Send { id } => {
            let result = client.send_order(id).await?;
            print_result(&result, format);
        }
        OrdersCommand::Approve { id } => {
            let result = client.approve_order(id).await?;
            print_result(&result, format);
        }
        OrdersCommand::Reject { id } => {
            let result = client.reject_order(id).await?;
            print_result(&result, format);
        }
        OrdersCommand::Download { id, output } => {
            let bytes = client.download_order(id).await?;
            std::fs::write(output, &bytes)?;
            println!("Downloaded to {output}");
        }
        OrdersCommand::DownloadPrimerTemplate { output } => {
            let bytes = client.download_primer_template().await?;
            std::fs::write(output, &bytes)?;
            println!("Downloaded to {output}");
        }
        OrdersCommand::DownloadSequencingTemplate { output } => {
            let bytes = client.download_sequencing_template().await?;
            std::fs::write(output, &bytes)?;
            println!("Downloaded to {output}");
        }
        OrdersCommand::UploadPrimerExcel { file } => {
            let result = client.upload_primer_excel(file).await?;
            print_result(&result, format);
        }
        OrdersCommand::UploadSequencingExcel { file } => {
            let result = client.upload_sequencing_excel(file).await?;
            print_result(&result, format);
        }
    }
    Ok(())
}
