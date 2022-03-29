// use std::io;
mod conditions;
mod fun_and_pro;
mod primitive;
mod strings;
mod structs_traits_impl;
use structs_traits_impl::*;
mod tuples;
use tuples::*;
struct LiteRecord {
    // rust not support inheritance it support composion
    states: i128,
    records: Records,
}
fn main() {
    // main in reality is procedure because it not return anything
    // println!("Enter the number");

    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    // print!("You guessed: {}", guess);

    // primitive::primitve();
    // strings::strings();

    // let result = fun_and_pro::some_function(10.1, 10);
    // println!("Result: {}", result);

    // fun_and_pro::some_procedure(10.1, 10);

    // let string_slice_var: &str = "Hi Rust";
    // fun_and_pro::some_str_procedure(string_slice_var);

    // let string_var = String::from("Real String");
    // // fun_and_pro::some_str_procedure(string_var);  // compile error because string is immutable
    // fun_and_pro::some_str_procedure(&string_var); // coversion to &str
    // fun_and_pro::some_string_procedure(string_var);
    // fun_and_pro::some_string_procedure("Some String".to_string());

    // conditions::conditions();
    // tuples();
    let mut record_var = Records {
        data: "0x00as1dfdsdsfs".to_string(),
        timestamp: 1648544172,
    };

    record_var.timestamp = 1648544245;

    let mut record_var2 = Records {
        data: "0x00dfasd12asd".to_string(),
        ..record_var
    };

    let lite_record = LiteRecord {
        states: 123123,
        records: Records {
            data: "0X0013212ddfs".to_string(),
            timestamp: 1642266245,
        },
    };
}
