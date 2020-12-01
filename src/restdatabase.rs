#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
extern crate env_logger;
extern crate r2d2;
extern crate r2d2_sqlite;
pub extern crate rusqlite;
extern crate serde_json;

use rusqlite::{Connection, Result};

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

const SQL_INIT_WATCHLIST_DATABASE_MOVIE: &'static str = "CREATE TABLE IF NOT EXISTS user_watchlist_movie (
    user_id           INTEGER PRIMARY KEY,
    movie_id          INTEGER NOT NULL,
    FOREIGN KEY (movie_id) REFERENCES Movies(movie_id),
    FOREIGN KEY (user_id) REFERENCES Login(user_id)
);";

const SQL_INIT_WATCHLIST_DATABASE_SERIES: &'static str = "CREATE TABLE IF NOT EXISTS user_watchlist_series (
    user_id           INTEGER PRIMARY KEY,
    series_id         INTEGER NOT NULL,
    FOREIGN KEY (series_id) REFERENCES Series(series_id),
    FOREIGN KEY (user_id) REFERENCES Login(user_id)
);";


const SQL_LOGIN_INIT_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Login (
    user_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,
    username     TEXT NOT NULL UNIQUE,
    email        TEXT NOT NULL,
    age          INTEGER NOT NULL,
    password     TEXT NOT NULL,     
    FOREIGN KEY (user_id) REFERENCES Login(user_id)
);";

const SQL_INIT_WATCHED_DATABASE_MOVIE: &'static str = "CREATE TABLE IF NOT EXISTS watched_movie (
    user_id      INTEGER PRIMARY KEY NOT NULL,
    movie_id     INTEGER NOT NULL,
    date         TEXT NOT NULL,     
    FOREIGN KEY (movie_id) REFERENCES Movies(movie_id),
    FOREIGN KEY (user_id) REFERENCES Login(user_id)
)";

const SQL_INIT_WATCHED_DATABASE_SERIES: &'static str = "CREATE TABLE IF NOT EXISTS watched_series (
    user_id      INTEGER PRIMARY KEY NOT NULL,
    series_id    INTEGER NOT NULL,
    FOREIGN KEY (series_id) REFERENCES Series(series_id),
    FOREIGN KEY (user_id) REFERENCES Login(user_id)
)";

const SQL_INIT_REVIEW_DATABASE: &'static str = "CREATE TABLE IF NOT EXISTS Review (
    user_id      INTEGER NOT NULL,
    movie_id     INTEGER,
    series_id    INTEGER,
    rating       INTEGER,
    review       TEXT,
    FOREIGN KEY (series_id) REFERENCES Series(series_id),
    FOREIGN KEY (user_id) REFERENCES Login(user_id),
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

    //conn.execute(
    //    "INSERT INTO Movies VALUES (1, \"chalo\", \"action\", 4.5 )",
    //    &[],
    //)?;

    conn.execute(
        "INSERT INTO Movies VALUES (2, \"chale\", \"scifi\", 4.5 )",
        &[],
    )
}






