if valtousize(&dict) != null{
    println!("are you sure that you've finished practicing this kanji?(yes\\no)" );
    let mut inp = String::new();
    stdin().read_line(&mut inp).ok();
    
    match inp.trim() {
        "yes" => {
            let mut ind: usize =(serde_json::to_string(&dict["pending"]).unwrap()).parse().unwrap();
            dict["used"].as_array_mut().unwrap().push(json!(ind));
            dict["pending"] = json!(null);
            write(path,&dict);
        },
        _ => println!("goodluck!"),
    }
} else {
    print!("you dont currently have a kanji to finish the practice!")
}
