fn main(){
    let s : String = String::from("string");
    let s = worlds(&s);
    println!("{s}");
}

fn worlds(s : &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        
    }
}