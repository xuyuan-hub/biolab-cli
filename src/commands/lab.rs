use std::sync::Arc;

use clap::{Args, Subcommand};

use crate::client::BiolabClient;
use crate::config::Config;
use crate::output::{
    print_lab_members, print_order_brief, print_result, print_stocks, OutputFormat,
};

#[derive(Args)]
pub struct LabArgs {
    #[command(subcommand)]
    pub command: LabCommand,
}

#[derive(Subcommand)]
pub enum LabCommand {
    /// Show lab information.
    Info,
    /// Create a lab.
    Create { name: String },
    /// Update lab settings with a JSON object.
    Update { data: String },
    /// List all orders in my lab.
    Orders,
    /// Show lab order statistics.
    OrdersStats,
    /// List shared lab inventory.
    Inventory,
    /// List lab members.
    Members,
    /// Update member role.
    UpdateRole { user_id: String, role: String },
    /// Remove a member.
    RemoveMember { user_id: String },
    /// Invite a member.
    Invite {
        email: String,
        #[arg(default_value = "member")]
        role: String,
    },
    /// List invitations.
    Invitations,
    /// Accept an invitation.
    AcceptInvite { invitation_id: String },
    /// Decline an invitation.
    DeclineInvite { invitation_id: String },
    /// Apply to join a lab.
    Join {
        lab_id: String,
        #[arg(default_value = "member")]
        role: String,
    },
    /// List join applications.
    Applications,
    /// Approve a join application.
    ApproveApp { application_id: String },
    /// Reject a join application.
    RejectApp { application_id: String },
    /// List approval rules.
    ApprovalRules,
    /// Add an approval rule from a JSON object.
    AddRule { data: String },
    /// Remove an approval rule.
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
        LabCommand::Orders => {
            let orders = client.list_lab_orders().await?;
            match format {
                OutputFormat::Json => print_result(&orders, format),
                OutputFormat::Text => {
                    if orders.is_empty() {
                        println!("No lab orders");
                    } else {
                        for order in &orders {
                            print_order_brief(order);
                        }
                    }
                }
            }
        }
        LabCommand::OrdersStats => {
            let stats = client.get_lab_order_stats().await?;
            print_result(&stats, format);
        }
        LabCommand::Inventory => {
            let stocks = client.list_lab_inventory().await?;
            match format {
                OutputFormat::Json => print_result(&stocks, format),
                OutputFormat::Text => print_stocks(&stocks),
            }
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
