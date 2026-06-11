use crate::api_response::{envelope_data, extract_object, extract_paginated, PaginatedList};
use crate::client::BiolabClient;
use crate::errors::BiolabError;
use crate::services::{empty_body, path_segment_encode, single_field_body};
use crate::types::{Application, ApprovalRule, Invitation, Lab, LabMember, Order, Stock};

impl BiolabClient {
    pub async fn get_lab(&self) -> Result<Lab, BiolabError> {
        let resp: serde_json::Value = self.http.get("/lab").await?;
        extract_object(resp)
    }

    pub async fn create_lab(&self, name: &str) -> Result<Lab, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post("/lab/create", &single_field_body("name", name))
            .await?;
        extract_object(resp)
    }

    pub async fn update_lab(&self, data: &serde_json::Value) -> Result<Lab, BiolabError> {
        let resp: serde_json::Value = self.http.patch("/lab", data).await?;
        extract_object(resp)
    }

    pub async fn list_lab_orders(&self) -> Result<PaginatedList<Order>, BiolabError> {
        let resp: serde_json::Value = self.http.get(lab_orders_path()).await?;
        extract_paginated(resp)
    }

    pub async fn get_lab_order_stats(&self) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.get(lab_order_stats_path()).await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_lab_inventory(&self) -> Result<PaginatedList<Stock>, BiolabError> {
        let resp: serde_json::Value = self.http.get(lab_inventory_path()).await?;
        extract_paginated(resp)
    }

    pub async fn list_lab_members(&self) -> Result<PaginatedList<LabMember>, BiolabError> {
        let resp: serde_json::Value = self.http.get("/lab/members").await?;
        extract_paginated(resp)
    }

    pub async fn update_member_role(
        &self,
        user_id: &str,
        role: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .patch(&member_path(user_id), &single_field_body("role", role))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn remove_member(&self, user_id: &str) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.delete(&member_path(user_id)).await?;
        Ok(envelope_data(resp))
    }

    pub async fn invite_member(
        &self,
        email: &str,
        role: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post("/lab/invite", &invite_body(email, role))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_invitations(&self) -> Result<PaginatedList<Invitation>, BiolabError> {
        let resp: serde_json::Value = self.http.get("/lab/invitations").await?;
        extract_paginated(resp)
    }

    pub async fn accept_invitation(
        &self,
        invitation_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &invitation_action_path(invitation_id, "accept"),
                &empty_body(),
            )
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn decline_invitation(
        &self,
        invitation_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &invitation_action_path(invitation_id, "decline"),
                &empty_body(),
            )
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn apply_to_join_lab(
        &self,
        lab_id: &str,
        role: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(&join_lab_path(lab_id), &single_field_body("role", role))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_applications(&self) -> Result<PaginatedList<Application>, BiolabError> {
        let resp: serde_json::Value = self.http.get("/lab/applications").await?;
        extract_paginated(resp)
    }

    pub async fn approve_application(
        &self,
        app_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(&application_action_path(app_id, "approve"), &empty_body())
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn reject_application(&self, app_id: &str) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(&application_action_path(app_id, "reject"), &empty_body())
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_approval_rules(&self) -> Result<PaginatedList<ApprovalRule>, BiolabError> {
        let resp: serde_json::Value = self.http.get("/lab/approval-rules").await?;
        extract_paginated(resp)
    }

    pub async fn add_approval_rule(
        &self,
        data: &serde_json::Value,
    ) -> Result<ApprovalRule, BiolabError> {
        let resp: serde_json::Value = self.http.post("/lab/approval-rules", data).await?;
        extract_object(resp)
    }

    pub async fn remove_approval_rule(
        &self,
        rule_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.delete(&approval_rule_path(rule_id)).await?;
        Ok(envelope_data(resp))
    }
}

fn invite_body(email: &str, role: &str) -> serde_json::Value {
    serde_json::json!({ "email": email, "role": role })
}

fn member_path(user_id: &str) -> String {
    format!("/lab/members/{}", path_segment_encode(user_id))
}

fn lab_orders_path() -> &'static str {
    "/lab/orders"
}

fn lab_order_stats_path() -> &'static str {
    "/lab/orders/stats"
}

fn lab_inventory_path() -> &'static str {
    "/lab/inventory/stocks"
}

fn invitation_action_path(invitation_id: &str, action: &str) -> String {
    format!(
        "/lab/invitations/{}/{action}",
        path_segment_encode(invitation_id)
    )
}

fn join_lab_path(lab_id: &str) -> String {
    format!("/lab/join/{}", path_segment_encode(lab_id))
}

fn application_action_path(app_id: &str, action: &str) -> String {
    format!("/lab/applications/{}/{action}", path_segment_encode(app_id))
}

fn approval_rule_path(rule_id: &str) -> String {
    format!("/lab/approval-rules/{}", path_segment_encode(rule_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_lab_bodies() {
        assert_eq!(
            single_field_body("name", "BioLab"),
            serde_json::json!({ "name": "BioLab" })
        );
        assert_eq!(
            single_field_body("role", "admin"),
            serde_json::json!({ "role": "admin" })
        );
        assert_eq!(
            invite_body("pi@example.com", "member"),
            serde_json::json!({ "email": "pi@example.com", "role": "member" })
        );
        assert_eq!(empty_body(), serde_json::json!({}));
    }

    #[test]
    fn builds_lab_member_and_join_paths() {
        assert_eq!(member_path("user-1"), "/lab/members/user-1");
        assert_eq!(lab_orders_path(), "/lab/orders");
        assert_eq!(lab_order_stats_path(), "/lab/orders/stats");
        assert_eq!(lab_inventory_path(), "/lab/inventory/stocks");
        assert_eq!(join_lab_path("lab-1"), "/lab/join/lab-1");
        assert_eq!(approval_rule_path("rule-1"), "/lab/approval-rules/rule-1");
    }

    #[test]
    fn encodes_lab_path_segments() {
        assert_eq!(member_path("user 1/a"), "/lab/members/user%201%2Fa");
        assert_eq!(join_lab_path("lab 1/a"), "/lab/join/lab%201%2Fa");
    }

    #[test]
    fn builds_invitation_and_application_action_paths() {
        assert_eq!(
            invitation_action_path("inv-1", "accept"),
            "/lab/invitations/inv-1/accept"
        );
        assert_eq!(
            invitation_action_path("inv-1", "decline"),
            "/lab/invitations/inv-1/decline"
        );
        assert_eq!(
            application_action_path("app-1", "approve"),
            "/lab/applications/app-1/approve"
        );
        assert_eq!(
            application_action_path("app-1", "reject"),
            "/lab/applications/app-1/reject"
        );
    }
}
