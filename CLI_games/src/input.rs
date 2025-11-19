use std::io::Write;

pub fn input(oput:&str)->String{
    let mut input: String = String::new();
    print!("{}",oput);
    std::io::stdout().flush().unwrap();
    if std::io::stdin().read_line(&mut input).is_ok(){
        input.trim().to_string()
    }
    else{
        "".to_string()
    }
}