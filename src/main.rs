use serde::{Deserialize,Serialize};
use serde_json::{json, Value};
use ansi_term::Color::*;
use structopt::StructOpt;
use std::io::stdin;
use std::io::Read;
use std::fs::File;
use regex::Regex;
use rand::Rng;
use std::env;
use reqwest;
use std::fs;

mod daily;
//fn write(path: &str,dict:&Value){
//    let write = serde_json::to_string_pretty(dict).unwrap();
//    fs::write(path, write);
//
//}
//fn main() {
//    
//    let flags: Vec<String> = env::args().collect();
//    let null: usize = 3000;
//    
//    
//    
//    
//
//    
//    if flags[1] == "-d"{
//        
//    }else if flags[1] == "-f"{
//    }else if flags[1] == "-p"{
//    }else if flags[1] == "-s"{
//    }
//}
#[derive(Debug, StructOpt)]
#[structopt(name = "Kanji", about = "A dictionary for kanji")]
enum Opt {
	
    daily,
	//finish,
}

fn main() {
    let opt = Opt::from_args();

	match opt{
			Opt::daily => daily::daily(),
			//Opt::finish => finish::finish()
	}
}

