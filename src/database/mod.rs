use diesel::prelude::*;
use std::env;

// This module exports functions for data retention
// Design goal - allow to interchange between sqlite, postgres, etc
pub mod commands;
pub mod models;
pub mod queries;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
