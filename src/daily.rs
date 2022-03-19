use serde::{Deserialize,Serialize};
use serde_json::{json, Value};
use rand::Rng;

mod parsek::parsek;

fn daily(){
    let kl = parsek(); 
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
