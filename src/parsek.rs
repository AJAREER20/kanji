use std::io::{self, Write};

fn parsek()-> Vec<char>{

	//let kpathbuf = std::path::PathBuf::from("../../src/kanji.txt");
	//let file: String = fs::canonicalize(&kpathbuf).expect("Unable to open").into_os_string().into_string().unwrap();

    let findDirectory = Command::new("find")
                    .arg("/home")
                    .arg("-type")
                    .arg("d")
                    .arg("-name")
                    .arg("kanji")
                    .output()
                    .expect("command failed to start");
    let mut directory = String::from_utf8(findDirectory.stdout).unwrap();
    directory.pop();
    let file =  format!("{}/src/kanji.txt",directory);
	//let idk = std::path::PathBuf::from("./");
	//let file1: String = fs::canonicalize(&idk).expect("Unable to open").into_os_string().into_string().unwrap();
	//println!("{:?}",file1);

	//io::stdout().flush().unwrap();

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
