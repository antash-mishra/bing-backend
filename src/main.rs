#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
#[macro_use]extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate r2d2_sqlite;
extern crate r2d2;
extern crate env_logger;
pub extern crate rusqlite;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use r2d2::{Pool, PooledConnection};
use rocket_contrib::databases;
use rusqlite::{Connection, MappedRows, Result, Row, types::FromSql, types::ToSql};
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::{Rocket, State, response::{Debug, content}, routes};
use std::sync::Mutex;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::marker::Sync;
use rocket::fairing::AdHoc;
use std::sync::RwLock;


#[database("SqliteDbConn")] 
struct SqliteDbConn(rusqlite::Connection);

impl SqliteDbConn {
    pub fn connection(&mut self) -> &mut rusqlite::Connection {
        &mut self.0
    }
}

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

fn create_db(conn: &Connection, sql_content: String) -> Result<()> {
    conn.execute(   
        sql_content.as_str(), &[]
    ).expect("No table found");

    Ok(())

}

fn my_movies(conn: &Connection) -> Result<Json<Datas>> {
    let mut stmt = conn.prepare("SELECT * FROM Movies").expect("Movies not found");
    let mut all_movies =  stmt
        .query_map( &[], |row| {
            Movies {
                movie_id:  row.get(0),
                title: row.get(1),
                genre: row.get(2),
                imdb_rating: row.get(3)
            }
        }).unwrap()
        .into_iter()
        .collect::<Result<Vec<Movies>>>()?; 

        println!("{:?}", all_movies);


    Ok(Json(Datas {
        all_movies,
    }))
    
}

#[get("/movies")]
fn get_movies(conn: SqliteDbConn) -> Result<Json<Datas>> {
    println!("hey");

    my_movies(&*conn)
    //let movies_json = serde_json::to_string(&movies_iter).unwrap();
    //println!("{}", movies_json);        
}

fn main() {

    env_logger::init();

    let db_conn = Connection::open("file.db").unwrap();


    let mut sql_file_content = read_sql_from_file("all.sql"); 
    println!("{}", sql_file_content);
        
    create_db(&db_conn, sql_file_content).unwrap();
    println!("hello");

    rocket::ignite()
        .attach(SqliteDbConn::fairing())
        .mount("/", routes![get_movies])
        //.manage(Mutex::new(db_conn))
        .launch();  
    
}
