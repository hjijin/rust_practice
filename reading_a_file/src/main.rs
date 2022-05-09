// cargo run the numbers.txt

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    println!("With text: {}", contents);

    let arrs: Vec<&str> = contents
        .split(", ")
        .collect();

    assert_eq!(arrs, ["71", "43", "75", "22", "33", "13", "74", "27", "80", "3"]);

    let mut numbers = Vec::new();
    for arr in arrs {
        let my_u32: u32 = arr.parse().unwrap();
        numbers.push(my_u32);
    }

    assert_eq!(numbers, [71, 43, 75, 22, 33, 13, 74, 27, 80, 3]);

    numbers.sort();
    let numbers_str = format!("{:?}", numbers); 
    fs::write("./new.txt", numbers_str).expect("Unable to write file");

    println!("Sorted and written into file.");
}
