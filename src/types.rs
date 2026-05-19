use serde::{Deserialize, Serialize};

// ============================================================
// User types
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub full_name: String,
    pub email: String,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub lab_id: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub lab: Option<Lab>,
}

// ============================================================
// Order types
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub status: String,
    pub supplier_name: String,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_email: String,
    #[serde(default)]
    pub total_price: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub items: Vec<OrderItem>,
    #[serde(default)]
    pub primer_items: Vec<PrimerItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub primer_name: String,
    pub sequence: String,
    #[serde(default)]
    pub base_count: Option<u32>,
    #[serde(default)]
    pub purification_method: Option<String>,
    #[serde(default)]
    pub nmoles: Option<f64>,
    #[serde(default)]
    pub scale_od: Option<f64>,
    #[serde(default)]
    pub tube_count: Option<u32>,
    #[serde(default)]
    pub five_modification: Option<String>,
    #[serde(default)]
    pub three_modification: Option<String>,
    // sequencing specific
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub seq_vector: Option<String>,
    #[serde(default)]
    pub universal: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimerItem {
    pub primer_name: String,
    pub sequence: String,
    #[serde(default)]
    pub scale_od: Option<String>,
    #[serde(default)]
    pub purification_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrimerOrder {
    pub r#type: String,
    pub supplier_name: String,
    pub items: Vec<OrderItem>,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_email: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub company_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub invoice_title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub principal_investigator: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub payment_method: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub recipient_address: String,
    #[serde(default)]
    pub weekend_delivery: bool,
    #[serde(default)]
    pub partial_delivery: bool,
    #[serde(default)]
    pub confidential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSequencingOrder {
    pub r#type: String,
    pub supplier_name: String,
    pub items: Vec<OrderItem>,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_email: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub company_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub invoice_title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub principal_investigator: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub payment_method: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub recipient_address: String,
    #[serde(default)]
    pub weekend_delivery: bool,
    #[serde(default)]
    pub partial_delivery: bool,
    #[serde(default)]
    pub confidential: bool,
}

// ============================================================
// Template types
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub is_default: Option<bool>,
    #[serde(default)]
    pub company_name: Option<String>,
    #[serde(default)]
    pub invoice_title: Option<String>,
    #[serde(default)]
    pub principal_investigator: Option<String>,
    #[serde(default)]
    pub payment_method: Option<String>,
    #[serde(default)]
    pub recipient_address: Option<String>,
    #[serde(default)]
    pub customer_name: Option<String>,
    #[serde(default)]
    pub customer_phone: Option<String>,
    #[serde(default)]
    pub customer_email: Option<String>,
}

// ============================================================
// Inventory types
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: String,
    #[serde(default)]
    pub primer_name: Option<String>,
    #[serde(default)]
    pub remaining_quantity: Option<f64>,
    #[serde(default)]
    pub location_path: Option<String>,
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub r#type: String,
    pub quantity: f64,
    pub purpose: String,
    #[serde(default)]
    pub experiment_ref: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockStats {
    pub total: u64,
    pub low_stock: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub path: Option<String>,
}

// ============================================================
// Lab types
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lab {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub require_approval: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabMember {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    pub id: String,
    #[serde(default)]
    pub lab_name: String,
    pub invitee_email: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub invitee_email: String,
    pub role: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRule {
    pub id: String,
    #[serde(default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub max_price: Option<f64>,
    pub approver_role: String,
    #[serde(default)]
    pub sort_order: Option<u32>,
}
