use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{print_lab_members, print_result, OutputFormat};

#[derive(Args)]
pub struct LabArgs {
    #[command(subcommand)]
    pub command: LabCommand,
}

#[derive(Subcommand)]
pub enum LabCommand {
    /// 课题组信息
    Info,
    /// 创建课题组
    Create { name: String },
    /// 更新课题组设置
    Update { data: String },
    /// 成员列表
    Members,
    /// 修改成员角色
    UpdateRole { user_id: String, role: String },
    /// 移除成员
    RemoveMember { user_id: String },
    /// 邀请成员
    Invite {
        email: String,
        #[arg(default_value = "member")]
        role: String,
    },
    /// 查看邀请
    Invitations,
    /// 接受邀请
    AcceptInvite { invitation_id: String },
    /// 拒绝邀请
    DeclineInvite { invitation_id: String },
    /// 申请加入课题组
    Join {
        lab_id: String,
        #[arg(default_value = "member")]
        role: String,
    },
    /// 查看入组申请（PI）
    Applications,
    /// 批准申请（PI）
    ApproveApp { application_id: String },
    /// 拒绝申请（PI）
    RejectApp { application_id: String },
    /// 审批规则列表
    ApprovalRules,
    /// 添加审批规则
    AddRule { data: String },
    /// 删除审批规则
    RemoveRule { rule_id: String },
}

pub async fn run(
    args: &LabArgs,
    config: &Arc<Config>,
    format: &OutputFormat,
) -> anyhow::Result<()> {
    let client = BiolabClient::new(Arc::clone(config))?;

    match &args.command {
        LabCommand::Info => {
            let lab = client.get_lab().await?;
            print_result(&lab, format);
        }
        LabCommand::Create { name } => {
            let lab = client.create_lab(name).await?;
            print_result(&lab, format);
        }
        LabCommand::Update { data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let lab = client.update_lab(&data).await?;
            print_result(&lab, format);
        }
        LabCommand::Members => {
            let members = client.list_lab_members().await?;
            match format {
                OutputFormat::Json => print_result(&members, format),
                OutputFormat::Text => print_lab_members(&members),
            }
        }
        LabCommand::UpdateRole { user_id, role } => {
            let result = client.update_member_role(user_id, role).await?;
            print_result(&result, format);
        }
        LabCommand::RemoveMember { user_id } => {
            let result = client.remove_member(user_id).await?;
            print_result(&result, format);
        }
        LabCommand::Invite { email, role } => {
            let result = client.invite_member(email, role).await?;
            print_result(&result, format);
        }
        LabCommand::Invitations => {
            let invitations = client.list_invitations().await?;
            print_result(&invitations, format);
        }
        LabCommand::AcceptInvite { invitation_id } => {
            let result = client.accept_invitation(invitation_id).await?;
            print_result(&result, format);
        }
        LabCommand::DeclineInvite { invitation_id } => {
            let result = client.decline_invitation(invitation_id).await?;
            print_result(&result, format);
        }
        LabCommand::Join { lab_id, role } => {
            let result = client.apply_to_join_lab(lab_id, role).await?;
            print_result(&result, format);
        }
        LabCommand::Applications => {
            let applications = client.list_applications().await?;
            print_result(&applications, format);
        }
        LabCommand::ApproveApp { application_id } => {
            let result = client.approve_application(application_id).await?;
            print_result(&result, format);
        }
        LabCommand::RejectApp { application_id } => {
            let result = client.reject_application(application_id).await?;
            print_result(&result, format);
        }
        LabCommand::ApprovalRules => {
            let rules = client.list_approval_rules().await?;
            print_result(&rules, format);
        }
        LabCommand::AddRule { data } => {
            let data: serde_json::Value = serde_json::from_str(data)?;
            let rule = client.add_approval_rule(&data).await?;
            print_result(&rule, format);
        }
        LabCommand::RemoveRule { rule_id } => {
            let result = client.remove_approval_rule(rule_id).await?;
            print_result(&result, format);
        }
    }
    Ok(())
}
