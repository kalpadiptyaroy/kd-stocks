use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl};

use crate::schema::stock::dsl::*;
use crate::models::Stock;
use crate::database;

pub fn get_all_stock() -> Vec<Stock> {
    let connection = &mut database::establish_connection();
    
    let result = stock.load::<Stock>(connection).expect("Error loading stocks");
    return result;
}

pub fn get_stock_by_id(stock_id: i32) -> Result<Stock, Error> {
    let connection = &mut database::establish_connection();
    let result: Result<Stock, Error> = stock.find(stock_id).first::<Stock>(connection);
    return result;
}