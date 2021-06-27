use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::*;

pub trait BaseModel {
    fn created_at(&self) -> Option<u64>;
    fn updated_at(&self) -> Option<u64>;
    fn set_created_at(&mut self, created_at: u64);
    fn set_updated_at(&mut self, updated_at: u64);

    fn uid(&self) -> Option<String>;
}

#[derive(Queryable, Insertable, Identifiable, Debug, Serialize, Deserialize)]
pub struct BackupRecord {
    pub id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub customers: Option<String>,
    pub laundry_documents: Option<String>,
    pub laundry_records: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerJson {
    pub uid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

impl BaseModel for CustomerJson {
    fn created_at(&self) -> Option<u64> {
        self.created_at
    }

    fn set_created_at(&mut self, created_at: u64) {
        self.created_at = Some(created_at);
    }

    fn updated_at(&self) -> Option<u64> {
        self.updated_at
    }

    fn set_updated_at(&mut self, updated_at: u64) {
        self.updated_at = Some(updated_at);
    }

    fn uid(&self) -> Option<String> {
        self.uid.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LaundryRecordJson {
    uid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,

    #[serde(rename = "customerUuid")]
    pub customer_uuid: Option<u64>,

    #[serde(rename = "laundryDocumentUuid")]
    pub laundry_document_uuid: Option<u64>,

    pub weight: Option<f64>,
    pub price: Option<u64>,

    #[serde(rename = "type")]
    pub record_type: Option<u32>,
    pub start: Option<u64>,
    pub done: Option<u64>,
    pub received: Option<u64>,

    #[serde(rename = "ePayId")]
    pub e_pay_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LaundryDocumentJson {
    pub uid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,

    pub name: Option<String>,
    pub date: Option<u64>,
}
