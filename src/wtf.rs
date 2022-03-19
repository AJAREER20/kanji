fn write(path: &str,dict:&Value){
    let write = serde_json::to_string_pretty(dict).unwrap();
    fs::write(path, write);

}
