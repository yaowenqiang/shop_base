use crate::schema::items;

#[derive(Queryable, Clone, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub instock: i32,
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub price: i32,
    pub instock: i32,
}
