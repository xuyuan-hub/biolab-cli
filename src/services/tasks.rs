use crate::api_response::{envelope_data, extract_object, extract_paginated, PaginatedList};
use crate::client::BiolabClient;
use crate::errors::BiolabError;
use crate::services::url_encode;
use crate::types::{StaffAssignment, Task, TaskDocument, TaskResult, TaskType};

impl BiolabClient {
    pub async fn list_lab_task_types(
        &self,
        lab_id: Option<&str>,
    ) -> Result<PaginatedList<TaskType>, BiolabError> {
        let path = lab_task_types_path();
        let resp: serde_json::Value = if let Some(lab_id) = lab_id {
            self.http
                .get_with_headers(&path, &[("X-Current-Lab", lab_id)])
                .await?
        } else {
            self.http.get(&path).await?
        };
        extract_paginated(resp)
    }

    pub async fn list_lab_tasks(
        &self,
        skip: u32,
        limit: u32,
        lab_id: Option<&str>,
    ) -> Result<PaginatedList<Task>, BiolabError> {
        let path = lab_tasks_path(skip, limit);
        let resp: serde_json::Value = if let Some(lab_id) = lab_id {
            self.http
                .get_with_headers(&path, &[("X-Current-Lab", lab_id)])
                .await?
        } else {
            self.http.get(&path).await?
        };
        extract_paginated(resp)
    }

    pub async fn get_lab_task(
        &self,
        task_id: &str,
        lab_id: Option<&str>,
    ) -> Result<Task, BiolabError> {
        let path = lab_task_path(task_id);
        let resp: serde_json::Value = if let Some(lab_id) = lab_id {
            self.http
                .get_with_headers(&path, &[("X-Current-Lab", lab_id)])
                .await?
        } else {
            self.http.get(&path).await?
        };
        extract_object(resp)
    }

    pub async fn list_lab_task_documents(
        &self,
        task_id: &str,
        lab_id: Option<&str>,
    ) -> Result<PaginatedList<TaskDocument>, BiolabError> {
        let path = lab_task_documents_path(task_id);
        let resp: serde_json::Value = if let Some(lab_id) = lab_id {
            self.http
                .get_with_headers(&path, &[("X-Current-Lab", lab_id)])
                .await?
        } else {
            self.http.get(&path).await?
        };
        extract_paginated(resp)
    }

    pub async fn download_lab_task_document(
        &self,
        document_id: &str,
        lab_id: Option<&str>,
    ) -> Result<Vec<u8>, BiolabError> {
        let path = lab_task_document_download_path(document_id);
        if let Some(lab_id) = lab_id {
            self.http
                .download_bytes_with_headers(&path, &[("X-Current-Lab", lab_id)])
                .await
        } else {
            self.http.download_bytes(&path).await
        }
    }

    pub async fn list_lab_task_results(
        &self,
        task_id: &str,
        lab_id: Option<&str>,
    ) -> Result<PaginatedList<TaskResult>, BiolabError> {
        let path = lab_task_results_path(task_id);
        let resp: serde_json::Value = if let Some(lab_id) = lab_id {
            self.http
                .get_with_headers(&path, &[("X-Current-Lab", lab_id)])
                .await?
        } else {
            self.http.get(&path).await?
        };
        extract_paginated(resp)
    }

    pub async fn create_task(&self, data: &serde_json::Value) -> Result<Task, BiolabError> {
        let resp: serde_json::Value = self.http.post(tasks_path(), data).await?;
        extract_object(resp)
    }

    pub async fn update_task(
        &self,
        task_id: &str,
        data: &serde_json::Value,
    ) -> Result<Task, BiolabError> {
        let resp: serde_json::Value = self.http.patch(&task_path(task_id), data).await?;
        extract_object(resp)
    }

    pub async fn list_my_task_assignments(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<PaginatedList<StaffAssignment>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&staff_task_assignments_path(skip, limit))
            .await?;
        extract_paginated(resp)
    }

    pub async fn get_my_task_assignment(
        &self,
        assignment_id: &str,
    ) -> Result<StaffAssignment, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&staff_task_assignment_path(assignment_id))
            .await?;
        extract_object(resp)
    }

    pub async fn update_my_task_assignment_status(
        &self,
        assignment_id: &str,
        status: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .patch(
                &staff_task_assignment_status_path(assignment_id),
                &serde_json::json!({ "status": status }),
            )
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn submit_my_task_result(
        &self,
        assignment_id: &str,
        data: &serde_json::Value,
    ) -> Result<TaskResult, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(&staff_task_assignment_results_path(assignment_id), data)
            .await?;
        extract_object(resp)
    }

    pub async fn list_my_task_documents(
        &self,
        task_id: &str,
    ) -> Result<PaginatedList<TaskDocument>, BiolabError> {
        let resp: serde_json::Value = self.http.get(&staff_task_documents_path(task_id)).await?;
        extract_paginated(resp)
    }

    pub async fn download_my_task_document(
        &self,
        document_id: &str,
    ) -> Result<Vec<u8>, BiolabError> {
        self.http
            .download_bytes(&staff_task_document_download_path(document_id))
            .await
    }
}

fn tasks_path() -> &'static str {
    "/tasks"
}

fn task_path(task_id: &str) -> String {
    format!("/tasks/{}", url_encode(task_id))
}

fn lab_task_types_path() -> String {
    "/lab/tasks/task-types".to_string()
}

fn lab_tasks_path(skip: u32, limit: u32) -> String {
    format!("/lab/tasks?skip={skip}&limit={limit}")
}

fn lab_task_path(task_id: &str) -> String {
    format!("/lab/tasks/{}", url_encode(task_id))
}

fn lab_task_documents_path(task_id: &str) -> String {
    format!("{}/documents", lab_task_path(task_id))
}

fn lab_task_document_download_path(document_id: &str) -> String {
    format!("/lab/tasks/documents/{}/download", url_encode(document_id))
}

fn lab_task_results_path(task_id: &str) -> String {
    format!("{}/results", lab_task_path(task_id))
}

fn staff_task_assignments_path(skip: u32, limit: u32) -> String {
    format!("/staff/tasks/assignments?skip={skip}&limit={limit}")
}

fn staff_task_assignment_path(assignment_id: &str) -> String {
    format!("/staff/tasks/assignments/{}", url_encode(assignment_id))
}

fn staff_task_assignment_status_path(assignment_id: &str) -> String {
    format!("{}/status", staff_task_assignment_path(assignment_id))
}

fn staff_task_assignment_results_path(assignment_id: &str) -> String {
    format!("{}/results", staff_task_assignment_path(assignment_id))
}

fn staff_task_documents_path(task_id: &str) -> String {
    format!("/staff/tasks/{}/documents", url_encode(task_id))
}

fn staff_task_document_download_path(document_id: &str) -> String {
    format!(
        "/staff/tasks/documents/{}/download",
        url_encode(document_id)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_lab_task_paths() {
        assert_eq!(lab_task_types_path(), "/lab/tasks/task-types");
        assert_eq!(lab_tasks_path(10, 25), "/lab/tasks?skip=10&limit=25");
        assert_eq!(lab_task_path("task 1"), "/lab/tasks/task+1");
        assert_eq!(
            lab_task_documents_path("task 1"),
            "/lab/tasks/task+1/documents"
        );
        assert_eq!(
            lab_task_document_download_path("doc 1"),
            "/lab/tasks/documents/doc+1/download"
        );
        assert_eq!(lab_task_results_path("task 1"), "/lab/tasks/task+1/results");
    }

    #[test]
    fn builds_general_task_paths() {
        assert_eq!(tasks_path(), "/tasks");
        assert_eq!(task_path("task 1"), "/tasks/task+1");
    }

    #[test]
    fn builds_staff_task_paths() {
        assert_eq!(
            staff_task_assignments_path(0, 100),
            "/staff/tasks/assignments?skip=0&limit=100"
        );
        assert_eq!(
            staff_task_assignment_path("assignment 1"),
            "/staff/tasks/assignments/assignment+1"
        );
        assert_eq!(
            staff_task_assignment_status_path("assignment 1"),
            "/staff/tasks/assignments/assignment+1/status"
        );
        assert_eq!(
            staff_task_assignment_results_path("assignment 1"),
            "/staff/tasks/assignments/assignment+1/results"
        );
        assert_eq!(
            staff_task_documents_path("task 1"),
            "/staff/tasks/task+1/documents"
        );
        assert_eq!(
            staff_task_document_download_path("doc 1"),
            "/staff/tasks/documents/doc+1/download"
        );
    }
}
