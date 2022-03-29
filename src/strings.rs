use std::fmt::format;

pub fn strings() {
    let example_str: &str = "Hello World"; // grouping of characters in a string immutable
    println!("{}", example_str);
    let example_string: String = String::from("Hello World"); // grouping of characters in a string mutable`
                                                              // both String and &str are grouoing of characters (u8's)
                                                              //* Strings
                                                              // Heap
                                                              // Mutable
                                                              //* &str is more complex
                                                              // immubtable
                                                              // allocated on the stack, sometimes a heap reference, ->
                                                              // -> sometimes embedded in the code
                                                              // translate between String and &Str

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();
    // hardcode strings are called string literals
    let string_from_hardcoded: String = String::from("Some hardcoded string");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_string; // deref symbol
                                                 // point to the oringnal memory location of the string

    // let test = "test" + "test1";
    let combine_string_literals = ["test", "test1"].concat();

    let combine_with_format_macro = format!("{} {}", "test", "test1");

    let string_plus_str = example_string + example_str;
    println!("{}", string_plus_str);

    // leanr borrowing

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("World");
    mut_string.push_str("!");
    mut_string.push('z');

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;

    let str_from_substring: &str = &mut_string[2..5];
    let str_from_substring2: &str = &mut_string[..5];
    let str_from_substring3: &str = &mut_string[2..=6];
    println!("{}", str_from_substring);
    println!("{}", str_from_substring2);
    println!("{}", str_from_substring3);

    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(c) => println!("{}", c),
        None => println!("No character found"),
    }

    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}
