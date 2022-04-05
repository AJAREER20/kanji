use serde::{Deserialize,Serialize};
use serde_json::{json, Value};
use ansi_term::Color::*;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::fs;

include!("vtu.rs");
include!("parsek.rs");
include!("getinfo.rs");
pub fn printc(){
	let null: usize = 3000;
	let kl = parsek();

	let path = "src/used.json";
	let mut d= File::open(path).unwrap();
	let mut contents= String::new();
	d.read_to_string(&mut contents);
	
	let mut dict: Value = serde_json::from_str(&contents).unwrap();

	if valtousize(&dict) != null{
	    printk(valtousize(&dict),kl);
	} else {
	    println!("you do not currently have a kanji to practice!");
	}
}
