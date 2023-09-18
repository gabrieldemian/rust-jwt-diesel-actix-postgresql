use crate::db;
use crate::error::Error;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[primary_key]
    pub id: i32,
    pub password: String,
    pub username: String,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, Error> {
        let mut conn = db::connection()?;
        let users = users::table.load::<Self>(&mut conn)?;

        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, Error> {
        let mut conn = db::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&mut conn)?;

        Ok(user)
    }

    pub fn find_by_username(username: &str) -> Result<Self, Error> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::username.eq(username))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn create(username: &str, password: &str) -> Result<Self, Error> {
        let mut conn = db::connection()?;

        let user = diesel::insert_into(users::table)
            .values((users::password.eq(password), users::username.eq(username)))
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn update(id: i32, user: User) -> Result<Self, Error> {
        let mut conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, Error> {
        let mut conn = db::connection()?;
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&mut conn)?;

        Ok(res)
    }
}
