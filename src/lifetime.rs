// Lifetime
// - Rust allows one and only one owner of memory
// - Rust allows multiple references
// - Lifetimes enforces a piece of memory is still valid for a reference

fn main() {
    // let a;
    // {
    //     let b = String::from("Howdy");
    //     a = &b;
    // }
    // println!("{}", a);
    let some_int_var = 10;
    let result_ref = get_int_ref(&some_int_var);
    println!("{}", result_ref);
}

// fn get_int_ref() -> &i32 {
//     let a = 1;
//     &a
// }

fn get_int_ref<'a>(param_1: &'a i32) -> &'a i32 {
    param_1
}

// fn some_func<'a, 'b>(param_1: &'a str, param_2: &'b str, param_3: Vec<64>) -> &str {

// }

// Every input reference that doesn't have an explicitly defined lifetime, it will be given it's OWN lifetime by Rust in the background
