#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use failure::Error;
mod models;
mod schema;
use models::{Item, NewItem};
use schema::items;

pub struct Conn(PgConnection);

impl Conn {
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL")?;
        Ok(Conn(PgConnection::establish(&db_url)?))
    }
    pub fn put_itmes(
        &mut self,
        name: &str,
        description: &str,
        price: i32,
        instock: i32,
    ) -> Result<Item, Error> {
        let nit = NewItem {
            name,
            description,
            price,
            instock,
        };
        diesel::insert_into(items::table)
            .values(&nit)
            .get_result::<Item>(&mut self.0)
            .map_err(|x| x.into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
