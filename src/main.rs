use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    //Add values
    marks.insert("Rust", 100);
    marks.insert("Web Development", 80);
    marks.insert("Java", 90);
    marks.insert("Selenium", 100);

    //Find length of HashMapp
    println!("How many subjects have you studied? {}", marks.len());

    //Get a single value
    match marks.get("Java") {
        Some(mark) => println!("You got {} for Java!", mark),
        None => println!("You didn't study Java!")
 
    }

    //Remove a value
    marks.remove("Selenium");

    //Loop through HashMap
    for (subject, mark) in &marks {
        println!("For {} you got {}%!", subject, mark);
    }

    //Check for value
    println!("Did you study C++? {}", marks.contains_key("C++ Programming"));

    //key => value
}
