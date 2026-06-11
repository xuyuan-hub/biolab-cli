use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{
    print_paginated_items, print_pagination_metadata, print_result, print_stocks, OutputFormat,
};

#[derive(Args)]
pub struct InventoryArgs {
    #[command(subcommand)]
    pub command: InventoryCommand,
}

#[derive(Subcommand)]
pub enum InventoryCommand {
    /// List inventory stocks.
    List {
        #[arg(long)]
        primer_name: Option<String>,
        #[arg(long)]
        location_id: Option<String>,
        #[arg(long)]
        low_stock: bool,
    },
    /// Show inventory stock details.
    Get { id: String },
    /// Show inventory stock transactions.
    Transactions { id: String },
    /// Show inventory statistics.
    Stats,
    /// Show inventory preferences.
    Preferences,
    /// Update inventory preferences with a JSON object.
    SetPreferences { data: String },
    /// Check in stock.
    Checkin {
        id: String,
        #[arg(long)]
        quantity: f64,
        #[arg(long, default_value = "")]
        purpose: String,
    },
    /// Check out stock.
    Checkout {
        id: String,
        #[arg(long)]
        quantity: f64,
        #[arg(long, default_value = "")]
        purpose: String,
        #[arg(long, default_value = "")]
        experiment_ref: String,
    },
    /// List storage locations.
    Locations,
    /// Create a storage location.
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
                OutputFormat::Text => {
                    print_pagination_metadata(&stocks);
                    print_stocks(&stocks.items);
                }
            }
        }
        InventoryCommand::Get { id } => {
            let stock = client.get_stock(id).await?;
            print_result(&stock, format);
        }
        InventoryCommand::Transactions { id } => {
            let transactions = client.list_stock_transactions(id).await?;
            match format {
                OutputFormat::Json => print_result(&transactions, format),
                OutputFormat::Text => print_paginated_items(&transactions),
            }
        }
        InventoryCommand::Stats => {
            let stats = client.get_stock_stats().await?;
            print_result(&stats, format);
        }
        InventoryCommand::Preferences => {
            let preferences = client.get_inventory_preferences().await?;
            print_result(&preferences, format);
        }
        InventoryCommand::SetPreferences { data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let preferences = client.set_inventory_preferences(&data).await?;
            print_result(&preferences, format);
        }
        InventoryCommand::Checkin {
            id,
            quantity,
            purpose,
        } => {
            validate_quantity(*quantity)?;
            let stock = client.checkin(id, *quantity, purpose).await?;
            print_result(&stock, format);
        }
        InventoryCommand::Checkout {
            id,
            quantity,
            purpose,
            experiment_ref,
        } => {
            validate_quantity(*quantity)?;
            let stock = client
                .checkout(id, *quantity, purpose, experiment_ref)
                .await?;
            print_result(&stock, format);
        }
        InventoryCommand::Locations => {
            let locations = client.list_locations().await?;
            match format {
                OutputFormat::Json => print_result(&locations, format),
                OutputFormat::Text => print_paginated_items(&locations),
            }
        }
        InventoryCommand::CreateLocation { name, parent_id } => {
            let location = client.create_location(name, parent_id.as_deref()).await?;
            print_result(&location, format);
        }
    }
    Ok(())
}

fn validate_quantity(quantity: f64) -> anyhow::Result<()> {
    if !quantity.is_finite() || quantity <= 0.0 {
        anyhow::bail!("quantity must be a positive finite number");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_positive_finite_quantity() {
        validate_quantity(0.1).expect("positive quantity should be valid");
    }

    #[test]
    fn rejects_zero_negative_and_non_finite_quantities() {
        assert!(validate_quantity(0.0).is_err());
        assert!(validate_quantity(-1.0).is_err());
        assert!(validate_quantity(f64::INFINITY).is_err());
        assert!(validate_quantity(f64::NAN).is_err());
    }
}
