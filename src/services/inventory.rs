use crate::api_response::{envelope_data, extract_object, extract_paginated, PaginatedList};
use crate::client::BiolabClient;
use crate::errors::BiolabError;
use crate::services::{path_segment_encode, url_encode};
use crate::types::{InventoryItem, Location, Stock, StockOutResponse, StockStats, Transaction};

impl BiolabClient {
    pub async fn list_stocks(
        &self,
        name: Option<&str>,
        location_id: Option<&str>,
        low_stock: bool,
        skip: u32,
        limit: u32,
    ) -> Result<PaginatedList<Stock>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&list_stocks_path(name, location_id, low_stock, skip, limit))
            .await?;
        extract_paginated(resp)
    }

    pub async fn list_lab_stocks(
        &self,
        lab_id: Option<&str>,
        name: Option<&str>,
        location_id: Option<&str>,
        low_stock: bool,
        skip: u32,
        limit: u32,
    ) -> Result<PaginatedList<Stock>, BiolabError> {
        let path = list_lab_stocks_path(name, location_id, low_stock, skip, limit);
        let resp: serde_json::Value = match lab_id {
            Some(lab_id) => {
                self.http
                    .get_with_headers(&path, &[("X-Current-Lab", lab_id)])
                    .await?
            }
            None => self.http.get(&path).await?,
        };
        extract_paginated(resp)
    }

    pub async fn get_stock(&self, stock_id: &str) -> Result<Stock, BiolabError> {
        let resp: serde_json::Value = self.http.get(&stock_path(stock_id)).await?;
        extract_object(resp)
    }

    pub async fn list_stock_transactions(
        &self,
        stock_id: &str,
    ) -> Result<PaginatedList<Transaction>, BiolabError> {
        let resp: serde_json::Value = self.http.get(&stock_transactions_path(stock_id)).await?;
        extract_paginated(resp)
    }

    pub async fn get_stock_stats(&self) -> Result<StockStats, BiolabError> {
        let resp: serde_json::Value = self.http.get("/inventory/stats").await?;
        extract_object(resp)
    }

    pub async fn list_items(
        &self,
        skip: u32,
        limit: u32,
        search: Option<&str>,
        category: Option<&str>,
        supplier: Option<&str>,
        filters: Option<&str>,
    ) -> Result<PaginatedList<InventoryItem>, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&list_items_path(
                skip, limit, search, category, supplier, filters,
            ))
            .await?;
        extract_paginated(resp)
    }

    pub async fn get_item(&self, item_id: &str) -> Result<InventoryItem, BiolabError> {
        let resp: serde_json::Value = self.http.get(&item_path(item_id)).await?;
        extract_object(resp)
    }

    pub async fn create_item(
        &self,
        data: &serde_json::Value,
    ) -> Result<InventoryItem, BiolabError> {
        let resp: serde_json::Value = self.http.post("/inventory/items", data).await?;
        extract_object(resp)
    }

    pub async fn update_item(
        &self,
        item_id: &str,
        data: &serde_json::Value,
    ) -> Result<InventoryItem, BiolabError> {
        let resp: serde_json::Value = self.http.patch(&item_path(item_id), data).await?;
        extract_object(resp)
    }

    pub async fn disable_item(&self, item_id: &str) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.post(&disable_item_path(item_id), &()).await?;
        Ok(envelope_data(resp))
    }

    pub async fn create_stock(&self, data: &serde_json::Value) -> Result<Stock, BiolabError> {
        let resp: serde_json::Value = self.http.post("/inventory/stocks/batch", data).await?;
        extract_object(resp)
    }

    pub async fn inventory_summary(
        &self,
        skip: u32,
        limit: u32,
        search: Option<&str>,
        category: Option<&str>,
        filters: Option<&str>,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&inventory_summary_path(
                skip, limit, search, category, filters,
            ))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn list_inventory_transactions(
        &self,
        skip: u32,
        limit: u32,
        transaction_type: Option<&str>,
        item_id: Option<&str>,
        search: Option<&str>,
        filters: Option<&str>,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&inventory_transactions_path(
                skip,
                limit,
                transaction_type,
                item_id,
                search,
                filters,
            ))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn get_inventory_preferences(
        &self,
        workflow_type: Option<&str>,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .get(&preferences_path(workflow_type.unwrap_or("primer_store")))
            .await?;
        Ok(envelope_data(resp))
    }

    pub async fn set_inventory_preferences(
        &self,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, BiolabError> {
        let resp: serde_json::Value = self.http.put("/inventory/preferences", data).await?;
        Ok(envelope_data(resp))
    }

    pub async fn checkin(
        &self,
        stock_id: &str,
        quantity: f64,
        purpose: Option<&str>,
    ) -> Result<Transaction, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(&checkin_path(stock_id), &checkin_body(quantity, purpose))
            .await?;
        extract_object(resp)
    }

    pub async fn checkout(
        &self,
        stock_id: &str,
        quantity: f64,
        recipient: Option<&str>,
        purpose: Option<&str>,
        experiment_ref: Option<&str>,
        task_id: Option<&str>,
        part_id: Option<&str>,
        requirement_key: Option<&str>,
    ) -> Result<Transaction, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &checkout_path(stock_id),
                &checkout_body(
                    quantity,
                    recipient,
                    purpose,
                    experiment_ref,
                    task_id,
                    part_id,
                    requirement_key,
                ),
            )
            .await?;
        extract_object(resp)
    }

    pub async fn checkout_item(
        &self,
        item_id: &str,
        quantity: f64,
        recipient: Option<&str>,
        purpose: Option<&str>,
        experiment_ref: Option<&str>,
        task_id: Option<&str>,
        part_id: Option<&str>,
        requirement_key: Option<&str>,
    ) -> Result<StockOutResponse, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &checkout_item_path(item_id),
                &checkout_body(
                    quantity,
                    recipient,
                    purpose,
                    experiment_ref,
                    task_id,
                    part_id,
                    requirement_key,
                ),
            )
            .await?;
        extract_object(resp)
    }

    pub async fn adjust_stock(
        &self,
        stock_id: &str,
        quantity: f64,
        adjustment_type: &str,
        reason: Option<&str>,
    ) -> Result<Transaction, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &adjust_path(stock_id),
                &adjust_body(quantity, adjustment_type, reason),
            )
            .await?;
        extract_object(resp)
    }

    pub async fn transfer_stock(
        &self,
        stock_id: &str,
        location_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<Stock, BiolabError> {
        let resp: serde_json::Value = self
            .http
            .post(
                &transfer_path(stock_id),
                &transfer_body(location_id, reason),
            )
            .await?;
        extract_object(resp)
    }

    pub async fn list_locations(&self) -> Result<PaginatedList<Location>, BiolabError> {
        let resp: serde_json::Value = self.http.get("/inventory/locations").await?;
        extract_paginated(resp)
    }

    pub async fn create_location(
        &self,
        name: &str,
        parent_id: Option<&str>,
    ) -> Result<Location, BiolabError> {
        let data = create_location_body(name, parent_id);
        let resp: serde_json::Value = self.http.post("/inventory/locations", &data).await?;
        extract_object(resp)
    }
}

fn list_stocks_path(
    name: Option<&str>,
    location_id: Option<&str>,
    low_stock: bool,
    skip: u32,
    limit: u32,
) -> String {
    let mut params = vec![format!("skip={skip}"), format!("limit={limit}")];
    push_param(&mut params, "name", name);
    push_param(&mut params, "location_id", location_id);
    if low_stock {
        params.push("low_stock=true".to_string());
    }
    format!("/inventory/stocks?{}", params.join("&"))
}

fn list_lab_stocks_path(
    name: Option<&str>,
    location_id: Option<&str>,
    low_stock: bool,
    skip: u32,
    limit: u32,
) -> String {
    let path = list_stocks_path(name, location_id, low_stock, skip, limit);
    path.replacen("/inventory/stocks", "/lab/inventory/stocks", 1)
}

fn list_items_path(
    skip: u32,
    limit: u32,
    search: Option<&str>,
    category: Option<&str>,
    supplier: Option<&str>,
    filters: Option<&str>,
) -> String {
    let mut params = vec![format!("skip={skip}"), format!("limit={limit}")];
    push_param(&mut params, "search", search);
    push_param(&mut params, "category", category);
    push_param(&mut params, "supplier", supplier);
    push_param(&mut params, "filters", filters);
    format!("/inventory/items?{}", params.join("&"))
}

fn inventory_summary_path(
    skip: u32,
    limit: u32,
    search: Option<&str>,
    category: Option<&str>,
    filters: Option<&str>,
) -> String {
    let mut params = vec![format!("skip={skip}"), format!("limit={limit}")];
    push_param(&mut params, "search", search);
    push_param(&mut params, "category", category);
    push_param(&mut params, "filters", filters);
    format!("/inventory/summary?{}", params.join("&"))
}

fn inventory_transactions_path(
    skip: u32,
    limit: u32,
    transaction_type: Option<&str>,
    item_id: Option<&str>,
    search: Option<&str>,
    filters: Option<&str>,
) -> String {
    let mut params = vec![format!("skip={skip}"), format!("limit={limit}")];
    push_param(&mut params, "type", transaction_type);
    push_param(&mut params, "item_id", item_id);
    push_param(&mut params, "search", search);
    push_param(&mut params, "filters", filters);
    format!("/inventory/transactions?{}", params.join("&"))
}

fn preferences_path(workflow_type: &str) -> String {
    format!(
        "/inventory/preferences?workflow_type={}",
        url_encode(workflow_type)
    )
}

fn stock_path(stock_id: &str) -> String {
    format!("/inventory/stocks/{}", path_segment_encode(stock_id))
}

fn stock_transactions_path(stock_id: &str) -> String {
    format!(
        "/inventory/stocks/{}/transactions",
        path_segment_encode(stock_id)
    )
}

fn item_path(item_id: &str) -> String {
    format!("/inventory/items/{}", path_segment_encode(item_id))
}

fn disable_item_path(item_id: &str) -> String {
    format!("/inventory/items/{}/disable", path_segment_encode(item_id))
}

fn checkin_path(stock_id: &str) -> String {
    format!(
        "/inventory/stocks/{}/checkin",
        path_segment_encode(stock_id)
    )
}

fn checkout_path(stock_id: &str) -> String {
    format!(
        "/inventory/stocks/{}/checkout",
        path_segment_encode(stock_id)
    )
}

fn checkout_item_path(item_id: &str) -> String {
    format!("/inventory/items/{}/checkout", path_segment_encode(item_id))
}

fn adjust_path(stock_id: &str) -> String {
    format!("/inventory/stocks/{}/adjust", path_segment_encode(stock_id))
}

fn transfer_path(stock_id: &str) -> String {
    format!(
        "/inventory/stocks/{}/transfer",
        path_segment_encode(stock_id)
    )
}

fn checkin_body(quantity: f64, purpose: Option<&str>) -> serde_json::Value {
    let mut data = serde_json::json!({ "quantity": quantity });
    insert_optional_str(&mut data, "purpose", purpose);
    data
}

fn checkout_body(
    quantity: f64,
    recipient: Option<&str>,
    purpose: Option<&str>,
    experiment_ref: Option<&str>,
    task_id: Option<&str>,
    part_id: Option<&str>,
    requirement_key: Option<&str>,
) -> serde_json::Value {
    let mut data = serde_json::json!({ "quantity": quantity });
    insert_optional_str(&mut data, "recipient", recipient);
    insert_optional_str(&mut data, "purpose", purpose);
    insert_optional_str(&mut data, "experiment_ref", experiment_ref);
    insert_optional_str(&mut data, "task_id", task_id);
    insert_optional_str(&mut data, "part_id", part_id);
    insert_optional_str(&mut data, "requirement_key", requirement_key);
    data
}

fn adjust_body(quantity: f64, adjustment_type: &str, reason: Option<&str>) -> serde_json::Value {
    let mut data = serde_json::json!({
        "quantity": quantity,
        "type": adjustment_type,
    });
    insert_optional_str(&mut data, "reason", reason);
    data
}

fn transfer_body(location_id: Option<&str>, reason: Option<&str>) -> serde_json::Value {
    let mut data = serde_json::json!({});
    insert_optional_str(&mut data, "location_id", location_id);
    insert_optional_str(&mut data, "reason", reason);
    data
}

fn create_location_body(name: &str, parent_id: Option<&str>) -> serde_json::Value {
    let mut data = serde_json::json!({ "name": name });
    insert_optional_str(&mut data, "parent_id", parent_id);
    data
}

fn push_param(params: &mut Vec<String>, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        params.push(format!("{}={}", key, url_encode(value)));
    }
}

fn insert_optional_str(data: &mut serde_json::Value, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        data[key] = serde_json::Value::String(value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_stock_list_path_without_filters() {
        assert_eq!(
            list_stocks_path(None, None, false, 0, 100),
            "/inventory/stocks?skip=0&limit=100"
        );
    }

    #[test]
    fn builds_stock_list_path_with_encoded_filters() {
        assert_eq!(
            list_stocks_path(Some("primer A+B"), Some("loc-1"), true, 20, 50),
            "/inventory/stocks?skip=20&limit=50&name=primer+A%2BB&location_id=loc-1&low_stock=true"
        );
    }

    #[test]
    fn builds_lab_stock_list_path() {
        assert_eq!(
            list_lab_stocks_path(Some("dNTP"), None, false, 0, 10),
            "/lab/inventory/stocks?skip=0&limit=10&name=dNTP"
        );
    }

    #[test]
    fn builds_item_summary_and_transaction_paths() {
        assert_eq!(
            list_items_path(
                0,
                50,
                Some("enzyme"),
                Some("reagent"),
                Some("NEB"),
                Some(r#"[{"field":"unit"}]"#)
            ),
            "/inventory/items?skip=0&limit=50&search=enzyme&category=reagent&supplier=NEB&filters=%5B%7B%22field%22%3A%22unit%22%7D%5D"
        );
        assert_eq!(
            inventory_summary_path(0, 50, Some("enzyme"), Some("reagent"), None),
            "/inventory/summary?skip=0&limit=50&search=enzyme&category=reagent"
        );
        assert_eq!(
            inventory_transactions_path(0, 50, Some("checkout"), Some("item-1"), Some("PCR"), None),
            "/inventory/transactions?skip=0&limit=50&type=checkout&item_id=item-1&search=PCR"
        );
    }

    #[test]
    fn builds_stock_action_paths() {
        assert_eq!(stock_path("stock-1"), "/inventory/stocks/stock-1");
        assert_eq!(
            stock_transactions_path("stock-1"),
            "/inventory/stocks/stock-1/transactions"
        );
        assert_eq!(checkin_path("stock-1"), "/inventory/stocks/stock-1/checkin");
        assert_eq!(
            checkout_path("stock-1"),
            "/inventory/stocks/stock-1/checkout"
        );
        assert_eq!(adjust_path("stock-1"), "/inventory/stocks/stock-1/adjust");
        assert_eq!(
            transfer_path("stock-1"),
            "/inventory/stocks/stock-1/transfer"
        );
        assert_eq!(
            checkout_item_path("item-1"),
            "/inventory/items/item-1/checkout"
        );
    }

    #[test]
    fn encodes_inventory_path_segments() {
        assert_eq!(
            stock_transactions_path("stock 1/a"),
            "/inventory/stocks/stock%201%2Fa/transactions"
        );
        assert_eq!(
            disable_item_path("item 1/a"),
            "/inventory/items/item%201%2Fa/disable"
        );
    }

    #[test]
    fn builds_stock_change_bodies() {
        assert_eq!(
            checkin_body(2.5, Some("restock")),
            serde_json::json!({ "quantity": 2.5, "purpose": "restock" })
        );
        assert_eq!(
            checkout_body(
                1.0,
                Some("Alice"),
                Some("experiment"),
                Some("exp-7"),
                Some("task-1"),
                Some("part-1"),
                Some("pcr.dntp")
            ),
            serde_json::json!({
                "quantity": 1.0,
                "recipient": "Alice",
                "purpose": "experiment",
                "experiment_ref": "exp-7",
                "task_id": "task-1",
                "part_id": "part-1",
                "requirement_key": "pcr.dntp"
            })
        );
    }

    #[test]
    fn builds_adjust_transfer_and_location_bodies() {
        assert_eq!(
            adjust_body(-1.0, "loss", Some("tube leaked")),
            serde_json::json!({
                "quantity": -1.0,
                "type": "loss",
                "reason": "tube leaked"
            })
        );
        assert_eq!(
            transfer_body(Some("loc-2"), Some("new freezer")),
            serde_json::json!({ "location_id": "loc-2", "reason": "new freezer" })
        );
        assert_eq!(
            create_location_body("Shelf 1", Some("freezer-a")),
            serde_json::json!({ "name": "Shelf 1", "parent_id": "freezer-a" })
        );
    }
}
