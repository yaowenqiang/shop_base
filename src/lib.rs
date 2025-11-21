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

    pub fn find_item(&mut self, name: &str, limit: i64) -> Result<Vec<Item>, Error> {
        let lname = format!("%{}%", name);
        items::table
            .filter(items::name.ilike(lname))
            .order(items::id.desc())
            .limit(limit)
            .load(&mut self.0)
            .map_err(|e| e.into())
    }

    pub fn set_stock(&mut self, id: i32, mut n: i32, rel: bool) -> Result<Item, Error> {
        if rel {
            let items: Vec<Item> = items::table.find(id).for_update().load(&mut self.0)?;
            n += items[0].instock;
        }
        diesel::update(items::table::find(items::table, id))
            .set(items::instock.eq(n))
            .get_result(&mut self.0)
            .map_err(|e| e.into())
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
