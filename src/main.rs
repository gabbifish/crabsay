#[macro_use] extern crate text_io;

extern crate clap;
use clap::{Arg, App};

static FERRIS: &str = "
           /   
          /
    _~^~^~_
\\) /  o o  \\ (/
  '_   -   _'
  / '-----' \\
";

fn main() {
    let matches = App::new("crabsay")
                            .version("0.1")
                            .author("Gabbi Fisher <gabbifisher@gmail.com>")
                            .about("*snip snip*")
                            .arg(Arg::with_name("word-wrap")
                                .short("w")
                                .takes_value(true)
                                .value_name("WIDTH")
                                .help("Specify the width of the text in the speech bubble"))
                            .arg(Arg::with_name("no-word-wrap")
                                .short("n")
                                .takes_value(false)
                                .help("Disable word wrap; this is good for ascii art"))
                            .arg(Arg::with_name("INPUT")
                                .help("Input for Ferris to say back at you!")
                                .index(1))
                            .get_matches();

    let mut input_text = matches.value_of("INPUT").unwrap_or("").to_string();
    if input_text == "" {
        input_text = read!("{}\r");
    }

    let no_wrap = match matches.occurrences_of("no-word-wrap") {
        1 => true,
        _ => false,
    };

    let mut text_width;
    if matches.is_present("word-wrap") {
        text_width = matches.value_of("word-wrap").unwrap().parse::<u32>().unwrap();
    }
    else {
        text_width = input_text.len() as u32
    }

    // Parse text into individual lines (represented by strings in vectors)
    let mut parsed_text = Vec::new();
    if !no_wrap {
        parse_input_wrap(&mut parsed_text, input_text, text_width);
    }
    else {
        text_width = parse_input_nowrap(&mut parsed_text, input_text);
        // Need to update text_width as well.
    }

    print_output(parsed_text, text_width);
}

fn parse_input_wrap<'a>(parsed_text: &mut Vec<String>, input_text: String, max_width: u32) {
    let input_len = input_text.len();
    for i in (0..input_len).step_by(max_width as usize) {
        parsed_text.push(input_text[i as usize..std::cmp::min(i + max_width as usize, input_len)].to_string());
    }
}

fn parse_input_nowrap<'a>(parsed_text: &mut Vec<String>, input_text: String) -> u32 {
    let mut lines = input_text.lines();
    let mut max_width = 0;
    loop {
        match lines.next() {
            Some(x) => {
                if x.len() > max_width {
                    max_width = x.len();
                }
                parsed_text.push(x.to_string());
            },
            None => { break }
        }
    }
    return max_width as u32
}

fn print_output(output_text: Vec<String>, max_width: u32) {
    // Construct appropriately sized "bubble" for output text, using line wrapping
    print_top_bottom(max_width);
    println!(); // Ensure body text appears on next line
    print_body(output_text, max_width);
    print_top_bottom(max_width);

    // Print ferris :)
    println!("{}", FERRIS);
}

fn print_top_bottom(max_width: u32) {
    // add 4 to max_width to account for | / \ < > edge characters and one space of buffer on each side
    let border = String::from_utf8(vec![b'-'; (max_width+4) as usize]).unwrap();
    print!("  {}", border);
}

fn print_body(output_text: Vec<String>, max_width: u32) {
    // Handle one-line case
    if output_text.len() == 1 {
        let padding = String::from_utf8(vec![b' '; max_width as usize - output_text[0].len()+2]).unwrap();
        println!(" < {}{} >", &output_text[0], padding)
    }
    // Handle 2+ line case
    else {
        let mut line_counter = 0;
        for line in &output_text {
            let padding = String::from_utf8(vec![b' '; max_width as usize - line.len()+2]).unwrap();
            if line_counter == 0 {
                println!(" / {}{} \\", &line, padding);
            }
            else if line_counter == output_text.len() - 1 {
                println!(" \\ {}{} /", &line, padding);
            }
            else {
                println!(" | {}{} |", &line, padding);
            }
            line_counter = line_counter + 1;
        }
    }
}

// TODO: fix
// fn whitespace<'a>(text_len: usize, max_width: u32) -> &'a str {
//     return &String::from_utf8(vec![b' '; max_width as usize - text_len]).unwrap();
// }

// fn get_current_location<'a>() -> &'a str {
//     // todo: get current location from ipinfo.io for default
//     return "DEFAULT LOCATION :)";
// }
