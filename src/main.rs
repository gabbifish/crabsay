extern crate clap;
use clap::{Arg, App};

static FERRIS: &str = "
           /   
       🎩  /
    _~^~^~_
\\) /  o o  \\ (/ ☕️
  '_   -   _'
  / '-----' \\
";

fn main() {
    let matches = App::new("crabsay")
                            .version("0.1")
                            .author("Gabbi Fisher <gabbifisher@gmail.com>")
                            .about("*snip snip*")
                            .arg(Arg::with_name("width")
                                .short("w")
                                .takes_value(true)
                                .default_value("F")
                                .value_name("WIDTH")
                                .help("Specify the width of the text in the speech bubble"))
                            .arg(Arg::with_name("LOCATION")
                                .help("The location whose weather to look up. If not specified, defaults to current location")
                                .index(1))
                            .get_matches();

    let text_width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    println!("{}", text_width);

    // let location = matches.value_of("LOCATION").unwrap_or_else(get_current_location);
    println!("{}", get_current_location());

    // todo: replace with weather fetching logic, which will handle formatting response lines into vec
    let mut parsed_text = Vec::new();
    parsed_text.push("Hello!");
    parsed_text.push("What's going on?");
    parsed_text.push("My name is Ferris!");

    print_output(parsed_text, 30);
}

fn print_output(output_text: Vec<&str>, max_width: u32) {
    // Construct appropriately sized "bubble" for output text, using line wrapping
    print_top_bottom(max_width);
    // print_body(output_text, max_width);
    print_top_bottom(max_width);

    // Print ferris :)
    println!("{}", FERRIS);
}

fn print_top_bottom(max_width: u32) {
    // add 4 to account for | / \ < > edge characters and one space of buffer on each side
    print!("{}", " ");
    for _n in 1..max_width+4 {
        print!("{}", "-");
    }
    print!("{}", "\n");
}

fn print_body(output_text: Vec<&str>, max_width: u32) {
    // allow for edge characters and 1 empty space buffer on each side of body text
    let inner_text_max_len = max_width;

    // // Handle one-line case
    if output_text.len() == 1 {
        print 
    }

}

fn get_current_location() -> String {
    // todo: get current location from ipinfo.io for default
    return "DEFAULT LOCATION :)".to_string();
}
