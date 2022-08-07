
use serde_json::{json, Value};
use std::process::Command;
use ansi_term::Color::*;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::fs;

include!("vtu.rs");
include!("parsek.rs");
include!("getinfo.rs");
pub fn search(kanji: char){
	let kl = parsek();

    let ind: usize = kl.iter().position(|&r| r == kanji).unwrap();
    
    printk(ind,kl);
}
