use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::intrinsics::prefetch_read_instruction;
// Module that currently is supposed to read data from file, and push it to database somehow (NOT IMPLEMENTED YET)
// Later, this will be fully-functional backend for crawlers to identify the language and talk to db

mod lib;

fn main() {
    file_to_db();
}

/// Example usage of the query sender
#[tokio::main]
async fn file_to_db() {
    println!("got here");
    if let Ok(lines) = read_lines("./data/100_lines_collected_data.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if index % 3 == 0 { // If it is a sequence of links, divided by space
                    println!("{}", ip);
                    let link_vector: Vec<&str> = ip.split(' ').collect::<Vec<&str>>(); // we don't need any memory efficiency here, it is all temporary

                    println!("{} {:?}", index, link_vector);
                    // Тут треба буде повставляти в базу
                }
                if index % 3 == 1 { // if it is just a text
                    let text: String = ip;
                    println!("{}", text);
                    let languages = lib::send_lang_detection_query(&text).await.unwrap();
                    // languages[0] = ("en".parse().unwrap(), 0.0001);
                    let dominant_lang = &languages[0];
                    println!("{:?}", dominant_lang);
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}







// fn change_and_get_first_element(a: &mut Vec<i32>) -> i32 {
//     a[0] = 4;
//     a[0]
// }


// fn plus_one(a: i32) -> i32 {
//     a + 1
// }
//
// fn testing() {
//     let mut c: [i32; 3] = [1, 2, 3];
//     c[0] = 102;
//     // let e = ["x"; 5]; // ["x", "x", "x", "x", "x"]
//     let square = |i| 3 * i * i; // { } are optional for single-lined closures
//     println!("{} {}", square(2), c[0]);
//     println!("{:#?}", c);
//     // let is_below_eighteen = if age < 18 { true } else { false }; // true
//
//     let mut v: Vec<i64> = vec![1, 2, 3];
//
//     for i in &v {
//         println!("A reference to {}", i);
//     }
//
//     for i in &mut v {
//         println!("A mutable reference to {}", i);
//     }
//
//     for i in v {
//         println!("Take ownership of the vector and its element {}", i);
//     }
// }
