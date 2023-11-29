use diesel::{prelude::Queryable, Selectable};
use rocket::serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[serde(crate = "rocket::serde")]
pub struct Stock {
    pub id: i32,
    pub nse_symbol: String,
    pub bse_symbol: String,
    pub name: String,
}