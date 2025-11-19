extern crate shop_db;
use failure::Error;

use shop_db::Conn;

fn main() -> Result<(), Error> {
    let mut conn = Conn::new()?;
    let r = conn.put_itmes("Car", "Drives around a bit", 3000, 10)?;
    println!("Added Item: {:?}", r);
    Ok(())
}
