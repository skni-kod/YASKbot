use crate::database::establish_connection;
use crate::database::models::*;
use anyhow::Result;
use diesel::prelude::*;

pub fn get_server_variables(server: i64) -> Result<Vec<ServerVariable>> {
    use crate::database::schema::server_variables::dsl::*;
    let conn = &mut establish_connection();
    let result = server_variables
        .filter(server_id.eq(server))
        .select(ServerVariable::as_select())
        .load(conn)?;

    Ok(result)
}

pub fn check_server(server: i64) -> Result<bool> {
    use crate::database::schema::servers::dsl::*;
    let conn = &mut establish_connection();
    let srv = servers
        .filter(server_id.eq(server))
        .select(Server::as_select())
        .load(conn)?;

    Ok(srv.len() > 0)
}
