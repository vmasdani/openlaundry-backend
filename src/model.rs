use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::*;

pub trait BaseModel {
    fn created_at(&self) -> Option<u64>;
    fn updated_at(&self) -> Option<u64>;
    fn deleted_at(&self) -> Option<u64>;

    fn set_created_at(&mut self, created_at: u64);
    fn set_updated_at(&mut self, updated_at: u64);
    fn set_deleted_at(&mut self, updated_at: u64);

    fn uuid(&self) -> Option<String>;
}

#[derive(Queryable, Insertable, Identifiable, Debug, Serialize, Deserialize)]
pub struct BackupRecord {
    pub id: Option<i32>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,

    pub customers: Option<String>,

    #[serde(rename = "laundryRecords")]
    pub laundry_records: Option<String>,

    #[serde(rename = "laundryDocuments")]
    pub laundry_documents: Option<String>,

    pub email: Option<String>,
    pub expenses: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerJson {
    pub uuid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,

    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<u64>,

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

    fn deleted_at(&self) -> Option<u64> {
        self.deleted_at
    }

    fn set_deleted_at(&mut self, deleted_at: u64) {
        self.deleted_at = Some(deleted_at);
    }

    fn uuid(&self) -> Option<String> {
        self.uuid.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaundryRecordJson {
    uuid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,

    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<u64>,

    #[serde(rename = "customerUuid")]
    pub customer_uuid: Option<String>,

    #[serde(rename = "laundryDocumentUuid")]
    pub laundry_document_uuid: Option<String>,

    pub weight: Option<f64>,
    pub price: Option<u64>,

    #[serde(rename = "type")]
    pub record_type: Option<u32>,
    pub start: Option<u64>,
    pub done: Option<u64>,
    pub received: Option<u64>,

    #[serde(rename = "ePayId")]
    pub e_pay_id: Option<u64>,
    pub wash: Option<bool>,
    pub dry: Option<bool>,
    pub iron: Option<bool>,
    pub note: Option<String>,
}

impl BaseModel for LaundryRecordJson {
    fn created_at(&self) -> Option<u64> {
        self.created_at
    }

    fn set_created_at(&mut self, created_at: u64) {
        self.created_at = Some(created_at);
    }

    fn updated_at(&self) -> Option<u64> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<u64> {
        self.deleted_at
    }

    fn set_deleted_at(&mut self, deleted_at: u64) {
        self.deleted_at = Some(deleted_at);
    }

    fn set_updated_at(&mut self, updated_at: u64) {
        self.updated_at = Some(updated_at);
    }

    fn uuid(&self) -> Option<String> {
        self.uuid.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaundryDocumentJson {
    pub uuid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,

    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<u64>,

    pub name: Option<String>,
    pub date: Option<u64>,
}

impl BaseModel for LaundryDocumentJson {
    fn created_at(&self) -> Option<u64> {
        self.created_at
    }

    fn set_created_at(&mut self, created_at: u64) {
        self.created_at = Some(created_at);
    }

    fn updated_at(&self) -> Option<u64> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<u64> {
        self.deleted_at
    }

    fn set_deleted_at(&mut self, deleted_at: u64) {
        self.deleted_at = Some(deleted_at);
    }

    fn set_updated_at(&mut self, updated_at: u64) {
        self.updated_at = Some(updated_at);
    }

    fn uuid(&self) -> Option<String> {
        self.uuid.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpenseJson {
    pub uuid: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<u64>,

    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<u64>,

    pub date: Option<u64>,
    pub amount: Option<f64>,
}

impl BaseModel for ExpenseJson {
    fn created_at(&self) -> Option<u64> {
        self.created_at
    }

    fn set_created_at(&mut self, created_at: u64) {
        self.created_at = Some(created_at);
    }

    fn updated_at(&self) -> Option<u64> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<u64> {
        self.deleted_at
    }

    fn set_deleted_at(&mut self, deleted_at: u64) {
        self.deleted_at = Some(deleted_at);
    }

    fn set_updated_at(&mut self, updated_at: u64) {
        self.updated_at = Some(updated_at);
    }

    fn uuid(&self) -> Option<String> {
        self.uuid.clone()
    }
}
