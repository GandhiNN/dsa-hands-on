fn basic_map() {
    let numbers = vec![11, 12, 13, 14];
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| n * n)
        .collect();
    println!("{:?}", result);
}

fn chaining_maps() {
    let numbers = vec![9, 8, 7, 6, 7];
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| n + 3)
        .map(|n| n * 3)
        .collect();
    println!("{:?}", result);
}

fn map_exec_with_collect() {
    let numbers = vec![11, 17, 19, 13];
    let mut number_of_times = 0;
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| {
            number_of_times += 1;
            return n * 10
        })
        .collect(); // this will execute the statement
    println!("number of execution: {} -> Result: {:?}", number_of_times, result);
}

fn map_exec_without_collect() {
    let numbers = vec![3, 5, 7, 9, 11];
    let mut number_of_times = 0;
    let result = numbers
        .iter()
        .map(|n| {
            number_of_times += 1;
            return n + 2
        });
    // We cannot use number_of_times because its ownership
    // has moved to result. We can only use it if we call
    // collect() on the result.
    println!("Result: {:?}", result);
}

fn transform_a_vector_of_strings_to_lowercase() {
    let words = vec!["Ngakan", "NYOMAN", "GandHi", "!"];
    println!("Words before map: {:?}", words);
    let lowercased_words: Vec<String> = words
        .iter()
        .map(|w| w.to_lowercase())
        .collect();
    println!("{:?}", lowercased_words);
}

fn count_characters_in_string() {
    let text = String::from("Ngakan Nyoman Gandhi");
    let result = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| (c, text.matches(c).count()))
        .collect::<std::collections::HashMap<_, _>>();
    println!("{:?}", result);
}

fn using_flat_map() {
    let str_numbers = vec!["1", "10", "20", "Ngakan Nyoman Gandhi"];
    let numbers: Vec<u32> = str_numbers
        .iter()
        .flat_map(|s| s.parse::<u32>())
        .collect();
    println!("{:?}", numbers);
}

fn main() {
    println!("Basic Map");    
    basic_map();
    println!("Chaining Map");
    chaining_maps();
    println!("Map execution with collect");
    map_exec_with_collect();
    println!("Map execution without collect");
    map_exec_without_collect();
    println!("Transform a vector of strings to lowercase");
    transform_a_vector_of_strings_to_lowercase();
    println!("Count characters in a string");
    count_characters_in_string();
    println!("Using Flat Map");
    using_flat_map();
}
