use rocket::get;
use diesel::{RunQueryDsl, QueryDsl};
use rocket::serde::json::Json;
use crate::schema::stock::dsl::*;
use crate::models::Stock;
use crate::database;

#[get("/stocks/getAll")]
pub fn index() -> Json<Vec<Stock>> {
    let connection = &mut database::establish_connection();
    
    let result = stock.load::<Stock>(connection).map(Json).expect("Error loading stocks");
    return result;
}

#[get("/stocks/get?<stock_id>")]
pub fn get_stock_by_id(stock_id: i32) -> Json<Stock> {
    let connection = &mut database::establish_connection();

    let result = 
        stock.find(stock_id)
             .first::<Stock>(connection)
             .map(Json)
             .expect("Error loading stocks");

    println!("{:?}", result);

    return result;
}