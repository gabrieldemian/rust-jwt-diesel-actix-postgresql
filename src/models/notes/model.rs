use crate::db;
use crate::error::Error;
use crate::schema::notes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, AsChangeset, Insertable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Note {
    #[primary_key]
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Note {
    pub fn find_all() -> Result<Vec<Self>, Error> {
        let mut conn = db::connection()?;
        let notes = notes::table.load::<Self>(&mut conn)?;

        Ok(notes)
    }

    pub fn find(id: i32) -> Result<Self, Error> {
        let mut conn = db::connection()?;
        let note = notes::table.filter(notes::id.eq(id)).first(&mut conn)?;

        Ok(note)
    }

    pub fn create(title: &str, content: &str) -> Result<Self, Error> {
        let mut conn = db::connection()?;

        let note = diesel::insert_into(notes::table)
            .values((notes::title.eq(title), notes::content.eq(content)))
            .get_result(&mut conn)?;

        Ok(note)
    }

    pub fn update(id: i32, title: &str, content: Option<&str>) -> Result<Self, Error> {
        let mut conn = db::connection()?;

        let mut note = diesel::update(notes::table)
            .filter(notes::id.eq(id))
            .set(notes::title.eq(title))
            .get_result(&mut conn)?;

        if let Some(content) = content {
            note = diesel::update(notes::table)
                .filter(notes::id.eq(id))
                .set(notes::content.eq(content))
                .get_result(&mut conn)?;
        }

        Ok(note)
    }

    pub fn delete(id: i32) -> Result<usize, Error> {
        let mut conn = db::connection()?;
        let res = diesel::delete(notes::table.filter(notes::id.eq(id))).execute(&mut conn)?;

        Ok(res)
    }
}
