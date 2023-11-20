// 'serde' dependency is mentioned in Cargo.toml file.

use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Stock {
    pub name: String,
    pub symbol: String
}

// Importing the services folder.
use crate::services;

#[get("/get-stock")]
pub fn get_stock() -> Json<Stock> {
    Json(services::stock::get_stock())
}