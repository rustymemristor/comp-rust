fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    let mut x: i32 = 10; // let mut if variable is mutable; else "let"
    println!("x: {x}");
    x = 20;
    println!("x: {x}");

    //interproduct function code
    println!("result: {}", interproduct(120, 78, 312));
}
