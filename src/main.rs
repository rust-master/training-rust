// use std::io;
mod conditions;
mod fun_and_pro;
mod primitive;
mod strings;
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

    conditions::conditions();
}
