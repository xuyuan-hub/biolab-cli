use crate::api_response::{envelope_data, extract_paginated, PaginatedList};
use crate::client::BiolabClient;
use crate::errors::BiolabError;
use crate::services::path_segment_encode;

impl BiolabClient {
    pub async fn list_projects(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self.http.get(&list_projects_path(skip, limit)).await?;
        extract_paginated(resp)
    }

    pub async fn get_project(&self, project_id: &str) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.get(&project_path(project_id)).await?;
        Ok(envelope_data(resp))
    }

    pub async fn create_project(
        &self,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.post("/projects", data).await?;
        Ok(envelope_data(resp))
    }

    pub async fn update_project(
        &self,
        project_id: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.patch(&project_path(project_id), data).await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_project_members(
        &self,
        project_id: &str,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self.http.get(&project_members_path(project_id)).await?;
        extract_paginated(resp)
    }

    pub async fn add_project_member(
        &self,
        project_id: &str,
        user_id: &str,
        role: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &project_members_path(project_id),
                &add_member_body(user_id, role),
            )
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn remove_project_member(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .delete(&project_member_path(project_id, user_id))
            .await?;
        Ok(envelope_data(resp))
    }
}

fn list_projects_path(skip: u32, limit: u32) -> String {
    format!("/projects?skip={skip}&limit={limit}")
}

fn project_path(project_id: &str) -> String {
    format!("/projects/{}", path_segment_encode(project_id))
}

fn project_members_path(project_id: &str) -> String {
    format!("/projects/{}/members", path_segment_encode(project_id))
}

fn project_member_path(project_id: &str, user_id: &str) -> String {
    format!(
        "/projects/{}/members/{}",
        path_segment_encode(project_id),
        path_segment_encode(user_id)
    )
}

fn add_member_body(user_id: &str, role: &str) -> serde_json::Value {
    serde_json::json!({
        "user_id": user_id,
        "role": role,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_project_paths() {
        assert_eq!(list_projects_path(20, 50), "/projects?skip=20&limit=50");
        assert_eq!(project_path("project-1"), "/projects/project-1");
        assert_eq!(
            project_members_path("project-1"),
            "/projects/project-1/members"
        );
        assert_eq!(
            project_member_path("project-1", "user-1"),
            "/projects/project-1/members/user-1"
        );
    }

    #[test]
    fn encodes_project_member_path_segments() {
        assert_eq!(
            project_member_path("project 1/a", "user 1/b"),
            "/projects/project%201%2Fa/members/user%201%2Fb"
        );
    }

    #[test]
    fn builds_add_member_body() {
        assert_eq!(
            add_member_body("user-1", "member"),
            serde_json::json!({ "user_id": "user-1", "role": "member" })
        );
    }
}
