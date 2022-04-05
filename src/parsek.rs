fn parsek()-> Vec<char>{
    let file = "src/kanji.txt";
    let contents = fs::read_to_string(file);
    let mut kl = Vec::new();
    for i in contents{
        for c in i.chars(){
            if c != '\r' && c != '\n'{
                kl.push(c);
            }
        }
    }
    return kl;
}
