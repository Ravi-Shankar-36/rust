use std::io;

fn main() {
    println!("Enter any number:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input!");
    let num :u32 = num.trim().parse().expect("Enter a number");
    println!("the input number is:{}", num);
}
