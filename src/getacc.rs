
fn getacc()->Value{
    let path = "src/used.json";
    let mut d= File::open(path).unwrap();
    let mut contents= String::new();
    d.read_to_string(&mut contents);
                                                                    
    let mut dict: Value = serde_json::from_str(&contents).unwrap();
    return dict;
}
