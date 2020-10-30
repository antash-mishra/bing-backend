#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
extern crate env_logger;
extern crate r2d2;
extern crate r2d2_sqlite;
pub extern crate rusqlite;
extern crate serde_json;

use r2d2::{Pool, PooledConnection};
use rocket::fairing::AdHoc;
use rocket::{
    response::{content, Debug},
    routes, Rocket, State, post,
    data::{FromData, FromDataSimple},
};
use rocket_contrib::databases;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rusqlite::{types::FromSql, types::ToSql, Connection, MappedRows, Result, Row};
use std::fs::File;
use std::io::prelude::*;
use std::marker::Sync;
use std::sync::Mutex;
use std::sync::RwLock;
use serde::Serialize;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use crate::SqliteDbConn;

const SQL_INIT_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Movies (
    movie_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    genre       TEXT NOT NULL,
    imdb_rating INTEGER NOT NULL
);";

const SQL_LOGIN_INIT_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Login (
    user_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,
    username     TEXT NOT NULL,
    email        TEXT NOT NULL,
    password     TEXT NOT NULL     
);";

pub fn create_db(conn: &Connection, sql_content: String) -> Result<usize> {
    conn.execute("DROP TABLE IF EXISTS Movies", &[])?;

    conn.execute(SQL_INIT_DATABASE, &[],)?;

    conn.execute(SQL_LOGIN_INIT_DATABASE, &[],)?;

    //conn.execute(
    //    "INSERT INTO Movies VALUES (1, \"chalo\", \"action\", 4.5 )",
    //    &[],
    //)?;

    conn.execute(
        "INSERT INTO Movies VALUES (2, \"chale\", \"scifi\", 4.5 )",
        &[],
    )
}






