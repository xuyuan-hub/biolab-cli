use crate::api_response::{envelope_data, extract_array, extract_paginated, PaginatedList};
use crate::client::BiolabClient;
use crate::errors::BiolabError;
use crate::services::{path_segment_encode, url_encode};

impl BiolabClient {
    pub async fn get_project_by_slug(&self, slug: &str) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.get(&project_by_slug_path(slug)).await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_project_germplasm(
        &self,
        slug: &str,
        skip: u32,
        limit: u32,
        search: Option<&str>,
        filters: Option<&str>,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_germplasm_list_path(
                slug, skip, limit, search, filters,
            ))
            .await?;
        extract_paginated(resp)
    }

    pub async fn create_project_germplasm(
        &self,
        slug: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.post(&project_germplasm_path(slug), data).await?;
        Ok(envelope_data(resp))
    }

    pub async fn get_project_germplasm(
        &self,
        slug: &str,
        germplasm_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_germplasm_detail_path(slug, germplasm_id))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn update_project_germplasm(
        &self,
        slug: &str,
        germplasm_id: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .patch(&project_germplasm_detail_path(slug, germplasm_id), data)
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn delete_project_germplasm(
        &self,
        slug: &str,
        germplasm_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .delete(&project_germplasm_detail_path(slug, germplasm_id))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_project_germplasm_sequencing_files(
        &self,
        slug: &str,
        germplasm_id: &str,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_germplasm_subresource_path(
                slug,
                germplasm_id,
                "sequencing-files",
            ))
            .await?;
        extract_paginated(resp)
    }

    pub async fn list_project_germplasm_stocks(
        &self,
        slug: &str,
        germplasm_id: &str,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_germplasm_subresource_path(
                slug,
                germplasm_id,
                "stocks",
            ))
            .await?;
        extract_paginated(resp)
    }

    pub async fn list_project_planting_orders(
        &self,
        slug: &str,
        skip: u32,
        limit: u32,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_planting_list_path(slug, skip, limit))
            .await?;
        extract_paginated(resp)
    }

    pub async fn create_project_planting_order(
        &self,
        slug: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.post(&project_planting_path(slug), data).await?;
        Ok(envelope_data(resp))
    }

    pub async fn get_project_planting_order(
        &self,
        slug: &str,
        order_id: &str,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_planting_detail_path(slug, order_id))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn update_project_planting_order(
        &self,
        slug: &str,
        order_id: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .patch(&project_planting_detail_path(slug, order_id), data)
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_project_planting_items(
        &self,
        slug: &str,
        order_id: &str,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_planting_subresource_path(slug, order_id, "items"))
            .await?;
        extract_paginated(resp)
    }

    pub async fn list_project_planting_harvests(
        &self,
        slug: &str,
        order_id: &str,
    ) -> Result<PaginatedList<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&project_planting_subresource_path(
                slug, order_id, "harvests",
            ))
            .await?;
        extract_paginated(resp)
    }

    pub async fn create_project_planting_harvest(
        &self,
        slug: &str,
        order_id: &str,
        data: &serde_json::Value,
    ) -> Result<Vec<serde_json::Value>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &project_planting_subresource_path(slug, order_id, "harvests"),
                data,
            )
            .await?;
        extract_array(resp)
    }
}

fn project_by_slug_path(slug: &str) -> String {
    format!("/projects/by-slug/{}", path_segment_encode(slug))
}

fn project_germplasm_path(slug: &str) -> String {
    format!("/project/{}/germplasm", path_segment_encode(slug))
}

fn project_germplasm_list_path(
    slug: &str,
    skip: u32,
    limit: u32,
    search: Option<&str>,
    filters: Option<&str>,
) -> String {
    let mut params = vec![format!("skip={skip}"), format!("limit={limit}")];
    if let Some(search) = search.filter(|value| !value.is_empty()) {
        params.push(format!("search={}", url_encode(search)));
    }
    if let Some(filters) = filters.filter(|value| !value.is_empty()) {
        params.push(format!("filters={}", url_encode(filters)));
    }
    format!("{}?{}", project_germplasm_path(slug), params.join("&"))
}

fn project_germplasm_detail_path(slug: &str, germplasm_id: &str) -> String {
    format!(
        "{}/{}",
        project_germplasm_path(slug),
        path_segment_encode(germplasm_id)
    )
}

fn project_germplasm_subresource_path(slug: &str, germplasm_id: &str, resource: &str) -> String {
    format!(
        "{}/{}",
        project_germplasm_detail_path(slug, germplasm_id),
        resource
    )
}

fn project_planting_path(slug: &str) -> String {
    format!("/project/{}/planting", path_segment_encode(slug))
}

fn project_planting_list_path(slug: &str, skip: u32, limit: u32) -> String {
    format!("{}?skip={skip}&limit={limit}", project_planting_path(slug))
}

fn project_planting_detail_path(slug: &str, order_id: &str) -> String {
    format!(
        "{}/{}",
        project_planting_path(slug),
        path_segment_encode(order_id)
    )
}

fn project_planting_subresource_path(slug: &str, order_id: &str, resource: &str) -> String {
    format!(
        "{}/{}",
        project_planting_detail_path(slug, order_id),
        resource
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_project_lookup_path() {
        assert_eq!(
            project_by_slug_path("ta shan"),
            "/projects/by-slug/ta%20shan"
        );
    }

    #[test]
    fn builds_germplasm_paths_with_encoded_queries() {
        assert_eq!(
            project_germplasm_list_path(
                "tashan",
                0,
                10,
                Some("rice A"),
                Some(r#"[{"field":"name","operator":"contains","value":"A"}]"#),
            ),
            "/project/tashan/germplasm?skip=0&limit=10&search=rice+A&filters=%5B%7B%22field%22%3A%22name%22%2C%22operator%22%3A%22contains%22%2C%22value%22%3A%22A%22%7D%5D"
        );
        assert_eq!(
            project_germplasm_detail_path("ta shan", "gp 1"),
            "/project/ta%20shan/germplasm/gp%201"
        );
        assert_eq!(
            project_germplasm_subresource_path("tashan", "gp-1", "stocks"),
            "/project/tashan/germplasm/gp-1/stocks"
        );
    }

    #[test]
    fn builds_planting_paths() {
        assert_eq!(
            project_planting_list_path("tashan", 0, 100),
            "/project/tashan/planting?skip=0&limit=100"
        );
        assert_eq!(
            project_planting_detail_path("ta shan", "order 1"),
            "/project/ta%20shan/planting/order%201"
        );
        assert_eq!(
            project_planting_subresource_path("tashan", "ord-1", "harvests"),
            "/project/tashan/planting/ord-1/harvests"
        );
    }
}
