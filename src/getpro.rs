fn getpro(allcap:&Regex,mut cap:String) -> String{
    cap = cap.replace(";","\n");
    let mut pro = "".to_string();
    for i in allcap.captures_iter(&cap){
        let change = i[1].to_string();
        if pro == ""{
            pro = change ;
        }else{
            pro = format!("{},  {}",pro,change);
        }
    }
    return pro;
} 
