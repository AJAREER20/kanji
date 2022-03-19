use reqwest;
use regex::Regex;
use ansi_term::Color::*;

fn printk(index:usize,kl:Vec<char>){
    
    let penk = kl[index].to_string();
    let link = format!("https://jisho.org/search/{}%20%23kanji",penk);

    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let response = client.get(link).send().unwrap();
    let html = response.text().unwrap();
    
    let capture = Regex::new("<div class=\"kanji-details__main-meanings\">\n\\s+(.+)\n\\s+</div>").unwrap();
    let kuncap = Regex::new("<dt>Kun:</dt>\n\\s+<dd class=\"kanji-details__main-readings-list\" lang=\"ja\">\n\\s+<a href=\"//jisho\\.org/search/.+\">.+</a>[&#\\d+;]?\\s+").unwrap();
    let oncap = Regex::new("<dt>On:</dt>\n\\s+<dd class=\"kanji-details__main-readings-list\" lang=\"ja\">\n\\s+<a href=\"//jisho\\.org/search/.+\">.+</a>[&#\\d+;]?\\s+").unwrap();
    let allcap = Regex::new("<a href=\"//jisho\\.org/search/.+\">(.+)</a>").unwrap();

    let kunstr:String = (&kuncap.captures(&html).unwrap()[0]).to_string();
    let onstr:String = (&oncap.captures(&html).unwrap()[0]).to_string();
    let meaning = &capture.captures(&html).unwrap()[1];
    println!("{}\nmeaning:\n    {}\nspelling:\n    Kun:  {}\n    On:  {}",
    Cyan.bold().paint(penk),
    &meaning,
    getpro(&allcap,kunstr),
    getpro(&allcap,onstr));
}
