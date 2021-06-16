fn main() {
    println!("Hello, world!");
    let test = add(5, 6);
    println!("{:?}", test);    
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}