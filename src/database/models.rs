use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::servers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Server {
    pub id: i32,
    pub server_id: i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::server_variables)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ServerVariable {
    pub id: i32,
    pub server_id: i64,
}
