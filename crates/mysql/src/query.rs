use crate::error::Error;
use crate::error::Error::*;
use crate::schema;
use crate::{models, MySqlPooledConnection};
use diesel::prelude::*;

pub fn songs(connection: &MySqlPooledConnection) -> Result<Vec<models::Song>, Error> {
    schema::songs::table
        .load(connection)
        .map_err(|e| DieselError(e))
}

pub fn hashes(connection: &MySqlPooledConnection) -> Result<Vec<models::Hash>, Error> {
    schema::hashes::table
        .load(connection)
        .map_err(|e| DieselError(e))
}

pub fn account_by_id(
    connection: &MySqlPooledConnection,
    user_id: i32,
) -> Result<models::User, Error> {
    schema::users::table
        .filter(schema::users::id.eq(user_id))
        .first(connection)
        .map_err(|e| DieselError(e))
}

pub fn account_by_google_id(
    connection: &MySqlPooledConnection,
    google_id: &String,
) -> Result<models::User, Error> {
    schema::users::table
        .filter(schema::users::google_id.eq(google_id))
        .first(connection)
        .map_err(|e| DieselError(e))
}

pub fn scores_by_user_id(
    connection: &MySqlPooledConnection,
    user_id: i32,
) -> Result<Vec<models::Score>, Error> {
    schema::scores::table
        .filter(schema::scores::user_id.eq(user_id))
        .load(connection)
        .map_err(|e| DieselError(e))
}

pub fn score_snaps_by_user_id(
    connection: &MySqlPooledConnection,
    user_id: i32,
) -> Result<Vec<models::ScoreSnap>, Error> {
    schema::score_snaps::table
        .filter(schema::score_snaps::user_id.eq(user_id))
        .load(connection)
        .map_err(|e| DieselError(e))
}
