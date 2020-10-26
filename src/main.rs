#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
#[macro_use]extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate env_logger;
pub extern crate rusqlite;
extern crate serde_json;

use rocket_contrib::databases;
use rusqlite::{Connection, MappedRows, Result, Row, types::FromSql, types::ToSql};
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::{Rocket, State, response::{Debug, content}, routes};
use std::sync::Mutex;
use rocket_contrib::json::Json;
use std::marker::Sync;


#[database("sqlite_logs")]
struct LogsDbConn(rusqlite::Connection);

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
fn get_movies(conn: LogsDbConn) -> Json<Datas> {

    let mut stmt = conn.prepare("SELECT * FROM MOVIES").unwrap();
    let movies_iter =  stmt.query_map( &[], |row| {
        Ok(Movies {
            movie_id:  row.get(0),
            title: row.get(1),
            genre: row.get(2),
            imdb_rating: row.get(3)
        })
    }).unwrap();

    let mut db_datas = Vec::new();

    for data in movies_iter {
        db_datas.push(data.unwrap());
    }



    Json(Datas {
        all_movies: db_datas,
    })
    
    //let movies_json = serde_json::to_string(&movies_iter).unwrap();
    //println!("{}", movies_json);        
}

fn main() {

    env_logger::init();

    
    let db_conn = Connection::open("file.db").unwrap();



    let sql_file_content = read_sql_from_file("all.sql"); 
    println!("{}", sql_file_content);
    db_conn.execute(   
        sql_file_content.as_str(), &[]
    ).expect("No table found");

    rocket::ignite()
        .attach(LogsDbConn::fairing())
        //.manage(Mutex::new(db_conn))
        .mount("/api", routes![get_movies])
        .launch();
    
}
