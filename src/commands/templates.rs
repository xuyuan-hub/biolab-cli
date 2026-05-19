use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{OutputFormat, print_result, print_templates};

#[derive(Args)]
pub struct TemplatesArgs {
    #[command(subcommand)]
    pub command: TemplatesCommand,
}

#[derive(Subcommand)]
pub enum TemplatesCommand {
    /// 模板列表
    List,
    /// 模板详情
    Get { id: String },
    /// 获取默认模板
    GetDefault { order_type: Option<String> },
    /// 创建模板（从 JSON 文件）
    Create { file: String },
    /// 更新模板（从 JSON 文件）
    Update { id: String, file: String },
    /// 删除模板
    Delete { id: String },
    /// 设为默认模板
    SetDefault { id: String },
}

pub async fn run(args: &TemplatesArgs, config: &Arc<Config>, format: &OutputFormat) -> anyhow::Result<()> {
    let client = BiolabClient::new(Arc::clone(config))?;

    match &args.command {
        TemplatesCommand::List => {
            let templates = client.list_templates().await?;
            match format {
                OutputFormat::Json => print_result(&templates, format),
                OutputFormat::Text => print_templates(&templates),
            }
        }
        TemplatesCommand::Get { id } => {
            let template = client.get_template(id).await?;
            print_result(&template, format);
        }
        TemplatesCommand::GetDefault { order_type } => {
            let template = client.get_default_template(order_type.as_deref()).await?;
            print_result(&template, format);
        }
        TemplatesCommand::Create { file } => {
            let content = std::fs::read_to_string(file)?;
            let data: serde_json::Value = serde_json::from_str(&content)?;
            let template = client.create_template(&data).await?;
            match format {
                OutputFormat::Json => print_result(&template, format),
                OutputFormat::Text => println!("模板已创建: {}", template.id),
            }
        }
        TemplatesCommand::Update { id, file } => {
            let content = std::fs::read_to_string(file)?;
            let data: serde_json::Value = serde_json::from_str(&content)?;
            let template = client.update_template(id, &data).await?;
            print_result(&template, format);
        }
        TemplatesCommand::Delete { id } => {
            let result = client.delete_template(id).await?;
            print_result(&result, format);
        }
        TemplatesCommand::SetDefault { id } => {
            let result = client.set_default_template(id).await?;
            match format {
                OutputFormat::Json => print_result(&result, format),
                OutputFormat::Text => println!("已设为默认模板: {id}"),
            }
        }
    }
    Ok(())
}
