
//use reqwest;
//use std::io::{Read, Result};
//use std::io::Error;
//use crate::Movies;

//impl Movies {
//    pub fn request_movies(self) -> Result<()> {
//        let mut moviesjson: Vec<Movies> = Vec::new();
//        let sites = "https://api.themoviedb.org/3/search/movie?api_key=3718fa836f765b876b4a98393770dcd4&language=en-US&query=".to_owned() + &self.title;
//        let mut body = String::new();
//        let json_body = reqwest::blocking::get(&sites).unwrap().read_to_string(&mut body); 
//        let datas = serde_json::from_str::<Movies>(&body).unwrap();
//
//        println!("{:?}", datas);
//        Ok(())
//    }
//}
