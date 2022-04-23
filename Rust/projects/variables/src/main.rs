fn main() {
    let tup :(i32, f64, char, bool) = (500, 4.123, 'C', true);
    let (x,y,z,l) = tup;
    println!("{} {} {} {}\n",x,y,z,l);
    println!("{}", tup.0);

    let strings = "This is a string";
    let arr = [1,2,3,4,5];
    println!("{} {}", strings, arr[0]);

    another_f();
}

fn another_f(){
    println!("another function");
}