use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::servers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Server {
    pub id: i32,
    pub server_id: i64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::servers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ServerVariables {
    pub id: i32,
    pub server_id: i64,
}