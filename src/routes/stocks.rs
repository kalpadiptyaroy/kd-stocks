use rocket::http::Status;
use rocket::get;
use rocket::serde::json::Json;

use crate::models::Stock;
use crate::services::stocks;

#[get("/stocks/getAll")]
pub fn index() -> Result<Json<Vec<Stock>>, Status> {
    let all_stocks = stocks::get_all_stock();
    if all_stocks.len() > 0 {
        return Ok(Json(all_stocks));
    }
    else {
        return Err(Status::NoContent);
    }
}

#[get("/stocks/get?<stock_id>")]
pub fn get_stock_by_id(stock_id: i32) -> Result<Json<Stock>, Status> {
    
    let result = stocks::get_stock_by_id(stock_id);

    if let Ok(result) = result {
        return Ok(Json(result));
    }
    else {
        return Err(Status::NotFound);
    }
}
