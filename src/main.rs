// use std::io;
mod conditions;
mod fun_and_pro;
mod primitive;
mod strings;
mod structs_traits_impl;
use structs_traits_impl::*;
mod tuples;
use tuples::*;
mod geo_map;
struct LiteRecord {
    // rust not support inheritance it support composion
    states: i128,
    records: Records,
}

impl Records {
    pub fn is_time_passed(&self, now: i32) -> bool {
        self.timestamp < now
    }
}

fn print_if_is_valid(check_me: &dyn RecordsTrait) {
    if check_me.is_valid() < 10 {
        println!("Valid")
    }
}

impl Default for Records {
    fn default() -> Self {
        Self {
            call_count: 0,
            data: "0x00as1dfdsdsfs".to_string(),
            timestamp: 1648544172,
        }
    }
}

//                    MACROS
// https://doc.rust-lang.org/rust-by-example/macros.html
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

// designators 
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    say_hello!();
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
    let mut record_var = Records::default();

    println!("{:?}", record_var);

    record_var.timestamp = 1648544245;

    let mut record_var2 = Records {
        data: "0x00dfasd12asd".to_string(),
        ..record_var
    };

    // let lite_record = LiteRecord {
    //     states: 123123,
    //     records: Records {
    //         data: "0X0013212ddfs".to_string(),
    //         timestamp: 1642266245,
    //     },
    // };

    let lite_record2 = LiteRecord {
        states: 123123,
        records: Records::new(1642266245),
    };

    let time_check = record_var2.time_check(1642266245);
    println!("Time Check: {}", time_check);
    let time_passed_check = record_var2.is_time_passed(1642266245);
    println!("Time Passed Check: {}", time_passed_check);

    let is_valid = record_var2.is_valid();
    println!("Is Valid: {}", is_valid);

    print_if_is_valid(&record_var2);

    let location = geo_map::get_lahore_location();
    println!(
        "Lahore location is at lat: {} and long: {}",
        location.lat, location.long
    );
}
