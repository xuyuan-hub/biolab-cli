use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{print_result, OutputFormat};

#[derive(Args)]
pub struct MeArgs {
    #[command(subcommand)]
    pub command: Option<MeCommand>,
}

#[derive(Subcommand)]
pub enum MeCommand {
    /// 更新个人信息
    Update {
        /// JSON 格式的更新字段，如 '{"phone_number":"138xxxx"}'
        #[arg(value_name = "JSON")]
        data: String,
    },
    /// 修改密码
    ChangePassword {
        /// 当前密码
        #[arg(long)]
        current: String,
        /// 新密码
        #[arg(long)]
        new: String,
    },
}

pub async fn run(args: &MeArgs, config: &Arc<Config>, format: &OutputFormat) -> anyhow::Result<()> {
    let client = BiolabClient::new(Arc::clone(config))?;

    match &args.command {
        None => {
            let user = client.get_me().await?;
            print_result(&user, format);
        }
        Some(MeCommand::Update { data }) => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let user = client.update_me(&data).await?;
            print_result(&user, format);
        }
        Some(MeCommand::ChangePassword { current, new }) => {
            let result = client.change_password(current, new).await?;
            print_result(&result, format);
        }
    }
    Ok(())
}
