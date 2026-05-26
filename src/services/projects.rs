use crate::api_response::{envelope_data, extract_array};
use crate::client::BiolabClient;
use crate::errors::BiolabError;

impl BiolabClient {
    pub async fn list_projects(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<Vec<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self.http.get(&list_projects_path(skip, limit)).await?;
        extract_array(resp)
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
    ) -> Result<Vec<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self.http.get(&project_members_path(project_id)).await?;
        extract_array(resp)
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
    format!("/projects/{project_id}")
}

fn project_members_path(project_id: &str) -> String {
    format!("/projects/{project_id}/members")
}

fn project_member_path(project_id: &str, user_id: &str) -> String {
    format!("/projects/{project_id}/members/{user_id}")
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
    fn builds_add_member_body() {
        assert_eq!(
            add_member_body("user-1", "member"),
            serde_json::json!({ "user_id": "user-1", "role": "member" })
        );
    }
}
