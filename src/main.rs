#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate env_logger;
extern crate rusqlite;
extern crate serde_json;
extern crate r2d2_sqlite;
extern crate r2d2;


use anyhow::Error;
use r2d2::Pool;
use rusqlite::{params, Connection, Result, types::ToSql, types::FromSql};
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::{response::{Debug, content}, Rocket, State, routes};
use rocket::request::FromRequest::from_request;
use std::sync::Mutex;
use rocket_contrib::json::Json;
use r2d2_sqlite::SqliteConnectionManager;
use std::marker::Sync;

type DbConn = Mutex<Connection>;

#[derive(Debug,Serialize, Deserialize)]
struct Movies {
    movie_id: i32,
    title: String,
    genre: String,
    imdb_rating: f64,
}
#[derive(Debug,Serialize, Deserialize)]
struct Datas {
    all_movies: Vec<Movies>
}


#[derive(Debug, Serialize, Deserialize)]
struct Tv_series {
    series_id: i32,
    title: String,
    genre: String,
    season: u64,
    episode: u64,
    imdb_rating: i32,
}

fn read_sql_from_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    contents
}

#[get("/movies")]
fn get_movies(db_conn: State<DbConn>, conn: Connection) -> Json<Movies> {

    let mut stmt = db_conn.prepare("SELECT * FROM Movies");
    //let movies_iter = 

        db_conn.lock()
            .expect("db connection lock")
            .query_map("SELECT * FROM Movies", params![], |row| {
                Ok(Movies {
                    movie_id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    genre: row.get(2).unwrap(),
                    imdb_rating: row.get(3).unwrap(),
                })
            }).unwrap();

    let mut db_datas: Vec<Movies> = Vec::new();
    for data in movies_iter {
        db_datas.push(data.unwrap());
    }
    
    let movies_json = serde_json::to_string(&movies_iter).unwrap();
    println!("{}", movies_json);
    Json(movies_iter)
        
}

fn main() {

    env_logger::init();

    
    let db_conn = Connection::open("file.db").unwrap();



    let sql_file_content = read_sql_from_file("all.sql"); 
    println!("{}", sql_file_content);
    
    db_conn.execute(   
        sql_file_content.as_str(), params![]
    ).expect("No table found");

    rocket::ignite()
        //.manage(Mutex::new(db_conn))
        .mount("/api", routes![get_movies])
        .launch();
    
}
