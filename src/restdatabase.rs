#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
extern crate env_logger;
extern crate r2d2;
extern crate serde_json;

use rocket::fairing::AdHoc;
use rocket::{
    response::{content, Debug},
    routes, Rocket, State, post,
    data::{FromData, FromDataSimple},
};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::fs::File;
use std::io::prelude::*;
use std::marker::Sync;
use std::sync::Mutex;
use std::sync::RwLock;
use serde::Serialize;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use crate::MysqlDbConn;
use mysql::*;
use mysql::params;
use mysql::prelude::*;

const  SQL_FOREIGN_KEY: &'static str = "PRAGMA synchronous = OFF";

const SQL_INIT_MOVIE_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Movies (
    movie_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    title        TEXT NOT NULL,
    genre        TEXT NOT NULL,
    imdb_rating  INTEGER
);";

const SQL_INIT_SERIES_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Series (
    series_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    title         TEXT NOT NULL,
    genre         TEXT NOT NULL,
    season        INTEGER,
    episode       INTEGER,
    imdb_rating   INTEGER NOT NULL
);";

const SQL_INIT_WATCHLIST_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS user_watchlist (
    user_id           INTEGER PRIMARY KEY AUTOINCREMENT,
    type              TEXT NOT NULL,
    movie_id          TEXT,
    series_id         INTEGER,
    FOREIGN KEY (movie_id) REFERENCES movie(movie_id),
    FOREIGN KEY (series_id) REFERENCES series(series_id)
);";

const SQL_LOGIN_INIT_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Login (
    user_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,
    username     TEXT NOT NULL,
    password     TEXT NOT NULL     
);";

pub fn create_db(sql_content: String) -> Result<()> {

    let url = "mysql://root:padmamishra@localhost:3307/dev";

    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let x = tx.query_drop("DROP TABLE IF EXISTS Movies");

    tx.query_drop(SQL_INIT_MOVIE_DATABASE)?;

    tx.query_drop(SQL_INIT_SERIES_DATABASE)?;

    tx.query_drop(SQL_INIT_WATCHLIST_DATABASE)?;

    tx.query_drop(SQL_LOGIN_INIT_DATABASE)?;

    tx.commit()?;
                                                                                    
    //conn.execute(
    //    "INSERT INTO Movies VALUES (1, \"chalo\", \"action\", 4.5 )",
    //    &[],
    //)?;

    Ok(())
}






