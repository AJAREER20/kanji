use std::env;
use std::fs::File;
use std::fs;
use serde::{Deserialize,Serialize};
use serde_json::{Result, Value,json};
use std::io::Read;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args[1] == "-d"{
        let file = "kanji.txt";
        let contents = fs::read_to_string(file);
        let mut kl = Vec::new();
        for i in contents{
            for c in i.chars(){
                kl.push(c);
            }
        }
        let path = "src/used.json";
        let mut d= File::open(path).unwrap();
        let mut contents= String::new();
        d.read_to_string(&mut contents);

        
        

        let dict: Value = serde_json::from_str(&contents).unwrap();
        let used = &dict["used"];
        let ls: Vec<u8>=serde_json::to_string(used).unwrap().as_bytes().to_vec();
    
        println!("{}\n{:?}",used,ls);

    }
}
