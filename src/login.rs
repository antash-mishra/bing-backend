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
use serde::Serialize;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use crate::SqliteDbConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login_datas {
    all_users: Vec<Login>,
}

impl Login_datas {
    pub fn add_user(new: &Login_datas, conn:&Connection) -> Result<()> {
        for k in &new.all_users {
            conn.execute("INSERT OR REPLACE INTO Login(name, username, password) values (?1, ?2, ?3)", 
                &[&k.name, &k.username, &k.password]
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    user_id: i32,
    name: String,
    username: String,   
    password: String,
}

pub fn my_users(conn: &Connection) -> Result<Json<Login_datas>> {
    let mut stmt = conn
        .prepare("SELECT * FROM Login")
        .expect("Couldn't find login");
    
    let mut all_users = stmt
        .query_map(&[], |row| Login {
            user_id: row.get(0),
            name: row.get(1),
            username: row.get(2),
            password: row.get(3),
        })
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<Login>>>()?;    
    println!("{:?}", all_users);

    Ok(Json(Login_datas{ all_users }))
}

#[get("/users")]
pub fn get_users(conn: SqliteDbConn) -> Result<Json<Login_datas>> {
        my_users(&conn)
}

#[post("/users", data = "<login_inputs>")]
pub fn post_users(login_inputs: Json<Login_datas>, conn: SqliteDbConn) -> Result<()> {
    format!("{:?}", login_inputs);
    let body = login_inputs.into_inner();
    Login_datas::add_user(&body, &conn)
}   