include!("vtu.rs");
fn finish(){
	let null:usize = 3000;

	let path = "src/used.json";
	let mut d= File::open(path).unwrap();
	let mut contents= String::new();
	d.read_to_string(&mut contents);
	
	let mut dict: Value = serde_json::from_str(&contents).unwrap();

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
}
