use std::io;
fn main() {
	println!("Write a number!");
	let mut number = String::new();
	io::stdin().read_line(&mut number).expect("IO Failure");
	let number: f64 = match number.trim().parse() {
		Ok(num) => num,
		Err(_) => panic!("Not a number!"),
	};
	
	println!("The number you typed is: {}", number);
}