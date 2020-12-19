#![feature(proc_macro_hygiene, decl_macro)]
#[warn(deprecated)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
<<<<<<< HEAD
=======
extern crate rocket_cors;
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
extern crate reqwest;

mod restdatabase;
mod login;
mod moviedb;

use rocket::fairing::AdHoc;
use rocket::{
    response::{content, Debug},
    routes, Rocket, State, post,
    data::{FromData, FromDataSimple},
};
use rocket_contrib::databases::{mysql::{Conn, *}};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::{env, fs::File};
use std::io::prelude::*;
use std::marker::Sync;
use std::sync::Mutex;
use std::sync::RwLock;
use serde::Serialize;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use reqwest::get;

<<<<<<< HEAD
#[database("mysql_logs")]
struct MysqlDbConn(mysql::Conn);
=======
#[database("SqliteDbConn")]
pub struct SqliteDbConn(Connection);
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de

#[derive(Debug, Serialize, Deserialize)]
struct Movies {
    movie_id: f64,
<<<<<<< HEAD
    title: String,
    genre: String,
    imdb_rating: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct AllMovies {
    movies: Vec<Movies>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Series {
    series_id: f64,
=======
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
    title: String,
    genre: String,
    season: f64,
    episode: f64,
    imdb_rating: f64,
}
#[derive(Debug, Serialize, Deserialize)]
<<<<<<< HEAD
struct AllSeries {
    series: Vec<Series>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddToWatchlist {
    types: String,
    movie_id: f64,
    series_id: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct user_watchlist {
    all_watchlist: Vec<AddToWatchlist>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQueries {
    Query: String,
}


//#[derive(Debug, Serialize, Deserialize)]
//struct Add_Tv_series {
//    id: f32,
//    title: String,
//    genre: String,
//    season: u64,
//    episode: u64,
//}

//#[derive(Debug, Serialize, Deserialize)]
//struct DataSeries {
//    all_Series: Vec<Add_Tv_series>,
//}


impl user_watchlist {
    pub fn add_watchlist(new: &user_watchlist, connection: &Connection) -> Result<()> {

        for j in &new.all_watchlist {
            connection.execute("INSERT OR REPLACE INTO user_watchlist(type, movie_id, series_id) values(?1, ?2, ?3)", 
            &[&j.types,  &j.movie_id, &j.series_id],
=======
struct AllMovies {
    movies: Vec<Movies>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Series {
    series_id: f64,
    title: String,
    genre: String,
    season: f64,
    episode: f64,
    imdb_rating: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct AllSeries {
    series: Vec<Series>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddToWatchlistMovie {
    user_id: f64,
    movie_id: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddToWatchlistSeries {
    user_id: f64,
    series_id: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct user_watchlist_movie {
    all_watchlist: Vec<AddToWatchlistMovie>,
}

#[derive(Debug, Serialize, Deserialize)]
struct user_watchlist_series {
    all_watchlist: Vec<AddToWatchlistSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQueries {
    Query: String,
}


impl user_watchlist_movie {
    pub fn add_watchlist(new: &user_watchlist_movie, conn: &Connection) -> Result<()> {

        for j in &new.all_watchlist {
            conn.execute("INSERT OR REPLACE INTO user_watchlist_movie(user_id, movie_id) values(?1, ?2)", 
            &[&j.user_id,  &j.movie_id],
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
            )?;
        }

        print!("hello");

<<<<<<< HEAD
=======
        Ok(())
    }
}

impl user_watchlist_series {
    pub fn add_watchlist(new: &user_watchlist_series, conn: &Connection) -> Result<()> {

        for j in &new.all_watchlist {
            conn.execute("INSERT OR REPLACE INTO user_watchlist_series(user_id, series_id) values(?1, ?2)", 
            &[&j.user_id,  &j.series_id],
            )?;
        }

        print!("hello");

>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
        Ok(())
    }
}

impl SearchQueries {
<<<<<<< HEAD
    pub fn query_search(&self, connection: &Connection) -> Result<()> {
        let mut search_results: Vec<Movies> = Vec::new();
        let mut body = String::new();
        let mut sites = "https://api.themoviedb.org/3/search/movie?api_key=3718fa836f765b876b4a98393770dcd4&language=en-US&query="
        .to_owned() + &self.Query;
=======
    pub fn query_search(&self, conn: &Connection) -> Result<()> {
        let mut search_results: Vec<Movies> = Vec::new();
        let mut body = String::new();
        let sites = "
            https://api.themoviedb.org/3/search/movie?api_key=3718fa836f765b876b4a98393770dcd4&language=en-US&query="
            .to_owned() + &self.Query;
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
        let json_body = reqwest::blocking::get(&sites).unwrap().read_to_string(&mut body); 
        let datas = serde_json::from_str::<AllMovies>(&body).unwrap();
        println!("{:?}", datas);
        Ok(())
    }
}


fn read_sql_from_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

<<<<<<< HEAD
fn get_user_watchlist(connection: &Connection) -> Result<Json<user_watchlist>> {
    let mut stmt = connection
        .prepare("SELECT * FROM user_watchlist")
        .expect("Not found");

    let mut all_watchlist = stmt
        .query_map(&[], |row| AddToWatchlist {
            types: row.get(0),
            movie_id: row.get(1),
            series_id: row.get(2),
        })
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<AddToWatchlist>>>()?;

        Ok(Json(user_watchlist {all_watchlist: all_watchlist}))
}

//fn my_movies(conn: &Connection) -> Result<Json<Datas>> {
//    let mut stmt = conn
//        .prepare("SELECT * FROM Movies")
//        .expect("Movies not found");
//    let mut all_movies = stmt
//        .query_map(&[], |row| Movies {
//            movie_id: row.get(0),
//            title: row.get(1),
//            genre: row.get(2),
//            imdb_rating: row.get(3),
//        })
//        .unwrap()
//        .into_iter()
//        .collect::<Result<Vec<Movies>>>()?;
//
//    println!("{:?}", all_movies);
//
//    Ok(Json(Datas { all_movies }))
//}


fn request_movies(connection: &Conn, all_movies: Vec<Movies>) -> Result<()> {
    let mut moviesjson: Vec<Movies> = Vec::new();
    let mut body = String::new();
    for j in all_movies {
        let sites = "https://api.themoviedb.org/3/search/movie?api_key=3718fa836f765b876b4a98393770dcd4&language=en-US&query=".to_owned() + &j.title;
        let json_body = reqwest::blocking::get(&sites).unwrap().read_to_string(&mut body); 
        let datas = serde_json::from_str::<Movies>(&body).unwrap();
        println!("{:?}", datas);
    }
    Ok(())
}

//#[get("/movies")]
//fn get_movies(conn: SqliteDbConn) -> Result<Json<Datas>> {
//    println!("hey");
//
//    my_movies(&conn)
//    //let movies_json = serde_json::to_string(&movies_iter).unwrap();
//    //println!("{}", movies_json);
//}
=======
fn get_user_watchlist_movie(conn: &Connection) -> Result<Json<user_watchlist_movie>> {
    let mut stmt = conn
        .prepare("SELECT * FROM user_watchlist")
        .expect("Not found");
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de

    let mut all_watchlist = stmt
        .query_map(&[], |row| AddToWatchlistMovie {
            user_id:row.get(0),
            movie_id: row.get(1),
        })
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<AddToWatchlistMovie>>>()?;

        Ok(Json(user_watchlist_movie {all_watchlist: all_watchlist}))


}

<<<<<<< HEAD
//#[post("/movies", data = "<user_input>")]
//fn post_movies(user_input: Json<Datas>,conn: SqliteDbConn) -> Result<()> {
//    format!("{:?}", user_input);
//    let body = user_input.into_inner();
//
//    Datas::add_movies(&body, &conn)
//}



#[get("/add_to_list")]
fn get_add_to_watchlist(connection: MysqlDbConn) -> Result<Json<user_watchlist>> {
    println!("add to watchlist");
    
    get_user_watchlist(&connection)
}

#[post("/add_to_list", data = "<user_input>")]
fn add_to_watchlist(user_input: Json<user_watchlist>, conn: MysqlDbConn) -> Result<()> {
    format!("{:?}", user_input);
    let movie_list_data = user_input.into_inner();

    user_watchlist::add_watchlist(&movie_list_data, &conn)
}

#[post("/add_to_list", data = "<user_input>")]
fn user_query(user_input: Json<SearchQueries>, conn: Conn) -> std::result::Result<(), mysql::Error> {
=======
fn get_user_watchlist_series(conn: &Connection) -> Result<Json<user_watchlist_series>> {
    let mut stmt = conn
        .prepare("SELECT * FROM user_watchlist")
        .expect("Not found");

    let mut all_watchlist = stmt
        .query_map(&[], |row| AddToWatchlistSeries {
            user_id:row.get(0),
            series_id: row.get(1),
        })
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<AddToWatchlistSeries>>>()?;

        Ok(Json(user_watchlist_series {all_watchlist: all_watchlist}))


}


//fn my_movies(conn: &Connection) -> Result<Json<Datas>> {
//    let mut stmt = conn
//        .prepare("SELECT * FROM Movies")
//        .expect("Movies not found");
//    let mut all_movies = stmt
//        .query_map(&[], |row| Movies {
//            movie_id: row.get(0),
//            title: row.get(1),
//            genre: row.get(2),
//            imdb_rating: row.get(3),
//        })
//        .unwrap()
//        .into_iter()
//        .collect::<Result<Vec<Movies>>>()?;
//
//    println!("{:?}", all_movies);
//
//    Ok(Json(Datas { all_movies }))
//}


fn request_movies(conn: &Connection, all_movies: Vec<Movies>) -> Result<()> {
    let mut moviesjson: Vec<Movies> = Vec::new();
    let mut body = String::new();
    for j in all_movies {
        let sites = "https://api.themoviedb.org/3/search/movie?api_key=3718fa836f765b876b4a98393770dcd4&language=en-US&query=".to_owned() + &j.title;
        let json_body = reqwest::blocking::get(&sites).unwrap().read_to_string(&mut body); 
        let datas = serde_json::from_str::<Movies>(&body).unwrap();
        println!("{:?}", datas);
    } 
    Ok(())
}


//#[get("/movies")]
//fn get_movies(conn: SqliteDbConn) -> Result<Json<Datas>> {
//    println!("hey");
//
//    my_movies(&conn)
//    //let movies_json = serde_json::to_string(&movies_iter).unwrap();
//    //println!("{}", movies_json);
//}


//#[post("/movies", data = "<user_input>")]
//fn post_movies(user_input: Json<Datas>,conn: SqliteDbConn) -> Result<()> {
//    format!("{:?}", user_input);
//    let body = user_input.into_inner();
//
//    Datas::add_movies(&body, &conn)
//}



#[get("/watch_list")]
fn get_add_to_watchlist(conn: SqliteDbConn) -> Result<Json<user_watchlist_series>> {
    println!("add to watchlist");
    
    get_user_watchlist_movie(&conn);
    get_user_watchlist_series(&conn)
}


#[post("/add_to_list_movie", data = "<user_input>")]
fn add_to_watchlist_movie(user_input: Json<user_watchlist_movie>, conn: SqliteDbConn) -> Result<()> {
    format!("{:?}", user_input);
    let movie_list_data = user_input.into_inner();

    user_watchlist_movie::add_watchlist(&movie_list_data, &conn)
}

#[post("/add_to_list_series", data = "<user_input>")]
fn add_to_watchlist_series(user_input: Json<user_watchlist_series>, conn: SqliteDbConn) -> Result<()> {
    format!("{:?}", user_input);
    let series_list_data = user_input.into_inner();

    user_watchlist_series::add_watchlist(&series_list_data, &conn)
}

#[post("/add_to", data = "<user_input>")]
fn user_query(user_input: Json<SearchQueries>, conn: SqliteDbConn) -> std::result::Result<(), rusqlite::Error> {
    
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
    format!("{:?}", user_input);

    let query_data = user_input.into_inner();

    SearchQueries::query_search(&query_data, &conn)
}

fn run_migrations(rocket: Rocket) -> std::result::Result<Rocket, Error> {
    let sql_file_content = read_sql_from_file("all.sql");
<<<<<<< HEAD
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let connection = MysqlDbConn::get_one(&rocket).expect("db conn");
    restdatabase::create_db(conn, sql_file_content).expect("as");
=======
    let mut conn = SqliteDbConn::get_one(&rocket).expect("db conn");
    restdatabase::create_db(&mut conn, sql_file_content).expect("as");
>>>>>>> 8f8e201b3c381086f89b56bd56795ac47f30c4de
    println!("done migr");
    Ok(rocket)
}


fn main() {
    env_logger::init();
    
    rocket::ignite()
        .attach(MysqlDbConn::fairing())
        .attach(AdHoc::on_attach("Migration", run_migrations))
        .mount("/", routes![
            //post_movies,
            login::get_users,
            login::post_users,
            add_to_watchlist_movie,
            add_to_watchlist_series,
            user_query,
            get_add_to_watchlist
        ])
        //.manage(Mutex::new(db_conn))
        .launch();
}

