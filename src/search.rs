mod parsek;
fn search(){
    let ind: usize = kl.iter().position(|&r| r == flags[2].parse::<char>().unwrap()).unwrap();
    
    printk(ind,kl);
}
