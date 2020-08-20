#[allow(dead_code)]

fn get_title() -> String {
	let mut the_title = String::from(env!("CARGO_PKG_NAME"));
	the_title.push_str(" (v");
	the_title.push_str(env!("CARGO_PKG_VERSION"));
	the_title.push_str("), ");
	the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
	return the_title;
}
#[allow(dead_code)]
fn parse_markdown_file() {

}
#[allow(dead_code)]
fn print_short_banner() {
println!("{}", get_title());
}
#[allow(dead_code)]
fn print_long_banner() {
print_short_banner();
 println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n", 
    env!("CARGO_PKG_AUTHORS"), 
    env!("CARGO_PKG_HOMEPAGE")
  );
	
}

fn usage() {

}

fn main() {
  usage();
let _args: Vec<String> = std::env::args().collect();

}


