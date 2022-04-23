fn main() {
    another_function(5,3);

    let x = 5;
    let y = {
        let x = 3;
        x+1
    };
    println!("{}", y);

    println!("{}", five());
}

fn another_function(x:i32, y:i32){
    println!("The value of x is : {}, {}", x, y);
}

fn five() -> i32{
    return 5
}