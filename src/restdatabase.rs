#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
extern crate env_logger;
extern crate r2d2;
extern crate serde_json;

<<<<<<< HEAD
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
=======
use rusqlite::{Connection, Result};

const  SQL_FOREIGN_KEY: &'static str = "PRAGMA synchronous = OFF";
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de

const SQL_LOGIN_INIT_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS User (
    username     TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
<<<<<<< HEAD
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
=======
    email        TEXT NOT NULL,
    age          INTEGER NOT NULL,
    password     TEXT NOT NULL
);";

const SQL_INIT_MOVIE_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Movies (
    movie_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    title        TEXT NOT NULL,
    genre        TEXT NOT NULL,
    imdb_rating  INTEGER NOT NULL,
    user_rating  DECIMAL(2,2)
);";

const SQL_INIT_SERIES_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Series (
    series_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    title         TEXT NOT NULL,
    genre         TEXT NOT NULL,
    season        INTEGER,
    episode       INTEGER,
    imdb_rating   INTEGER NOT NULL,
    user_rating   DECIMAL(2,2)
);";

const SQL_INIT_WATCHLIST_DATABASE_MOVIE: &'static str = "CREATE TABLE IF NOT EXISTS user_watchlist_movie (
    username          TEXT NOT NULL,
    movie_id          INTEGER NOT NULL,
    PRIMARY KEY (username,movie_id),
    FOREIGN KEY (movie_id) REFERENCES Movies(movie_id),
    FOREIGN KEY (username) REFERENCES User(username)
);";

const SQL_INIT_WATCHLIST_DATABASE_SERIES: &'static str = "CREATE TABLE IF NOT EXISTS user_watchlist_series (
    username          TEXT NOT NULL,
    series_id         INTEGER NOT NULL,
    PRIMARY KEY (username,series_id),
    FOREIGN KEY (series_id) REFERENCES Series(series_id),
    FOREIGN KEY (username) REFERENCES User(username)
);";


const SQL_INIT_WATCHED_DATABASE_MOVIE: &'static str = "CREATE TABLE IF NOT EXISTS watched_movie (
    username     TEXT NOT NULL,
    movie_id     INTEGER NOT NULL,
    date         TEXT NOT NULL,
    PRIMARY KEY (username,movie_id),
    FOREIGN KEY (movie_id) REFERENCES Movies(movie_id),
    FOREIGN KEY (username) REFERENCES User(username)
)";

const SQL_INIT_WATCHED_DATABASE_SERIES: &'static str = "CREATE TABLE IF NOT EXISTS watched_series (
    username     TEXT NOT NULL,
    series_id    INTEGER NOT NULL,
    date         TEXT,
    season       INTEGER,
    PRIMARY KEY (username,series_id),
    FOREIGN KEY (series_id) REFERENCES Series(series_id),
    FOREIGN KEY (username) REFERENCES User(username)
)";

const SQL_INIT_REVIEW_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Review_Movies (
    username     TEXT NOT NULL,
    movie_id     INTEGER,
    rating       INTEGER,
    review       TEXT,
    revmov_id    INTEGER AUTOINCREMENT,
    PRIMARY KEY (username,movie_id,revmov_id),
    FOREIGN KEY (username) REFERENCES User(username),
    FOREIGN KEY (movie_id) REFERENCES Movie(movie_id)
 )"; 

const SQL_INIT_REVIEW_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Review_Series (
    username     TEXT NOT NULL,
    series_id    INTEGER,
    revser_id    INTEGER AUTOINCREMENT,
    season       INTEGER,
    rating       INTEGER,
    review       TEXT,
    PRIMARY KEY (username,movie_id,revser_id),
    FOREIGN KEY (username) REFERENCES User(username),
    FOREIGN KEY (movie_id) REFERENCES Movie(movie_id)
 )"; 

pub fn create_db(conn: &mut Connection, sql_content: String) -> Result<usize> {

    conn.execute(SQL_FOREIGN_KEY, &[])?;
    
    let mut tx = conn.transaction()?;

    tx.execute("DROP TABLE IF EXISTS Movies", &[])?;

    tx.execute(SQL_LOGIN_INIT_DATABASE, &[])?;

    tx.execute(SQL_INIT_MOVIE_DATABASE, &[])?;

    tx.execute(SQL_INIT_SERIES_DATABASE, &[])?;

    tx.execute(SQL_INIT_WATCHLIST_DATABASE_MOVIE, &[])?;

    tx.execute(SQL_INIT_WATCHLIST_DATABASE_SERIES, &[])?;

    tx.execute(SQL_INIT_WATCHED_DATABASE_MOVIE, &[])?;

    tx.execute(SQL_INIT_WATCHED_DATABASE_SERIES, &[])?;

    tx.execute(SQL_INIT_REVIEW_DATABASE, &[])?;

    tx.commit()?;
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de

    tx.commit()?;
                                                                                    
    //conn.execute(
    //    "INSERT INTO Movies VALUES (1, \"chalo\", \"action\", 4.5 )",
    //    &[],
    //)?;

    Ok(())
}






