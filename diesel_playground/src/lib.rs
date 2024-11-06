use diesel::{Connection, PgConnection};
use eyre::{Context, Result};

pub mod models;
pub mod queries;
pub mod schema;

pub fn connect_to_db(database_url: &str) -> Result<PgConnection> {
    PgConnection::establish(database_url).context("connecting to database")
}
