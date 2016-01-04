//! bigdigits reads a number entered on the command line (as a string), and
//! outputs the same number onto the console using "big" digits.
//! That is, digits in the style of old big 20th century line printers that
//! used to print the job numbers and other identification in "big" digits
//! on the cover page.
//!
//! The idea for this program is taken from the book "Programming in GO"
//! It is an attempt to translate the GO program into Rust.

use std::env;
use std::path::Path;
use std::process;

const BIG_DIGITS: [[&'static str;7];10] = [
    ["  000  ",
     " 0   0 ",
     "0     0",
     "0     0",
     "0     0",
     " 0   0 ",
     "  000  "],
    [" 1 ", 
     "11 ", 
     " 1 ", 
     " 1 ", 
     " 1 ", 
     " 1 ", 
     "111"],
    [" 222 ",
     "2   2", 
     "   2 ", 
     "  2  ", 
     " 2   ", 
     "2    ", 
     "22222"],
    [" 333 ",
     "3   3", 
     "    3", 
     "  33 ", 
     "    3", 
     "3   3", 
     " 333 "],
    ["   4  ",
     "  44  ", 
     " 4 4  ", 
     "4  4  ", 
     "444444", 
     "   4  ",
     "   4  "],
    ["55555",
     "5    ", 
     "5    ", 
     " 555 ", 
     "    5", 
     "5   5", 
     " 555 "],
    [" 666 ",
     "6    ", 
     "6    ", 
     "6666 ", 
     "6   6", 
     "6   6", 
     " 666 "],
    ["77777",
     "    7", 
     "   7 ", 
     "  7  ", 
     " 7   ", 
     "7    ", 
     "7    "],
    [" 888 ",
     "8   8", 
     "8   8", 
     " 888 ", 
     "8   8", 
     "8   8", 
     " 888 "],
    [" 9999",
     "9   9", 
     "9   9", 
     " 9999", 
     "    9", 
     "    9", 
     "    9"],
    ];

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let program = Path::new(&program).file_name().unwrap();

	if args.len() == 1 {
		println!("usage: {:?} <whole-number>", program);
		process::exit(1);
	}
    
    let string_of_digits = args[1].clone();

    for row in 0..BIG_DIGITS[0].len() {
    	let mut line = String::new();

    	for column in string_of_digits.chars() {
    		let digit = column as i32 - '0' as i32;

    		if 0 <= digit && digit <= 9 {
	    		let digit = digit as usize;
    			line = format!("{}{}  ", line, BIG_DIGITS[digit][row].to_string());
    		} else {
    			println!("invalid whole-number");
    			process::exit(1);
    		}
    	}

    	println!("{}", line);
    }
}
