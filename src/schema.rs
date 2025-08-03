use serde::{Deserialize, Serialize};
// use crate::structs::Language;

// List
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub _page: Option<usize>,  // Currently unused
    pub _limit: Option<usize>, // Currently unused
}

// Create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBrickSchema {
    pub id: String,
    pub name: String,
    pub language: String,
    pub source_path: String,
}

// Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBrickSchema {
    pub id: Option<String>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub source_path: Option<String>,
}

// Delete -- Unused for now
// If I decide to use POST insead of DELETE
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteBrickSchema {
    pub id: Option<String>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub source_path: Option<String>,
}
