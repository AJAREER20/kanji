use serde::{Deserialize,Serialize};
use serde_json::{json, Value};
use ansi_term::Color::*;
use std::io::stdin;
use std::io::Read;
use std::fs::File;
use regex::Regex;
use rand::Rng;
use std::fs;

include!("parsek.rs");
include!("wtf.rs");
include!("vtu.rs");
include!("getinfo.rs");

pub fn daily(){
    let kl = parsek(); 

	let path = "src/used.json";
    let mut d= File::open(path).unwrap();
    let mut contents= String::new();
    d.read_to_string(&mut contents);

    let mut dict: Value = serde_json::from_str(&contents).unwrap();
    let mut rng = rand::thread_rng();
    while true{
        let randind = rng.gen_range(0..(kl.len()));
        if !dict["used"].as_array_mut().unwrap().contains(&json!(randind)) && dict["pending"] != json!(randind){
                
            println!("would you like to practice this \"{}\" kanji today?(y\\n\\exit)?",kl[randind]);
            let mut inp = String::new();
            stdin().read_line(&mut inp).ok();
            match inp.trim(){
                "y" => {
                    dict["pending"] = json!(randind);
                    write(path,&dict);
                    let ind = valtousize(&dict);
                    printk(ind,kl);
                    break;
                },
                "n" => continue,
                "exit" => break,
                _ => println!("please enter a viable choice!"),
            }
        }
    
    }
}
