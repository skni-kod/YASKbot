use crate::database::establish_connection;
use crate::database::models::*;
use anyhow::Result;
use diesel::prelude::*;

pub fn try_register_server(server: i64) -> Result<bool> {
    use crate::database::schema::servers::dsl::*;
    let conn = &mut establish_connection();
    let srv = servers
        .filter(server_id.eq(server))
        .select(Server::as_select())
        .load(conn)?;

    if srv.len() > 0 {
        return Ok(false);
    }

    let _ = diesel::insert_into(servers)
        .values((server_id.eq(server)))
        .execute(conn)?;

    return Ok(true);
}
