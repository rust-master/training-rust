#[derive(Debug)] // Procedural Macro
struct Data1 {
    pub a: i32,
    pub b: i32,
}

fn main() {
    let data = 10;
    print!("Rust");
    print!("Language");
    println!("Data is {}", data);
    println!("Data is {1} , {0}", 9, data);

    println!(
        "My name is {first_name} {last_name}",
        first_name = "M. Zaryab",
        last_name = "Rafique"
    );

    let data1 = Data1 { a: 1, b: 2 };
    let data2 = Data1 { a: 7, b: 5 };

    // Declarative Macro
    let some_formate_string = format!("{0:#?} \n {1:#?}", data1, data2);
    println!("{}", some_formate_string);
}
// You can create your own macros
// print! , println!
// format!
// use {} to print data
// order data with numeric placeholders like {0}{1}{2}...etc
// use explicitly name placeholders like {first_name}
// Print more complex struct data with the "Debug" macro
// "Pretty" format #
