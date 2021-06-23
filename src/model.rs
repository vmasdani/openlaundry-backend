use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CustomerJson {
    uid: Option<String>,

    #[serde(rename = "createdAt")]
    created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    updated_at: Option<u64>,
    name: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LaundryRecordJson {
    uid: Option<String>,

    #[serde(rename = "createdAt")]
    created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    updated_at: Option<u64>,

    #[serde(rename = "customerUuid")]
    customer_uuid: Option<u64>,

    #[serde(rename = "laundryDocumentUuid")]
    laundry_document_uuid: Option<u64>,

    weight: Option<f64>,
    price: Option<u64>,

    #[serde(rename = "type")]
    record_type: Option<u32>,
    start: Option<u64>,
    done: Option<u64>,
    received: Option<u64>,

    #[serde(rename = "ePayId")]
    e_pay_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct LaundryDocumentJson {
    uid: Option<String>,

    #[serde(rename = "createdAt")]
    created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    updated_at: Option<u64>,

    name: Option<String>,
    date: Option<u64>,
}
