extern crate reqwest;

use std::collections::HashMap;
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    languages_url: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repos: Vec<Repository> = reqwest::get("https://api.github.com/users/stevecondylios/starred")?
        .json()?;
    

    for repo in repos {
    	// println!("{}", repo.name);
    	let languages: HashMap<String, u32> = reqwest::get(&repo.languages_url)?.json()?;
        println!("{:#?}", repo);
    }


    Ok(())
}




// use serde::{Deserialize, Serialize};
// use serde_json::Result;

