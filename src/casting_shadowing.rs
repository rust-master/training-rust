// Rust Casting, Shadowing, Consts and Static

// CONSTANTS
const API_CODE: i64 = 100;

// STATIC
static mut MY_STATIC_VARIABLE: i32 = 10;

fn main() {
    let some_i32: i32 = 10;
    let some_i64: f64 = 20.2;

    let combined = some_i32 + some_i64 as i32;
    println!("{}", combined);

    let var_a: i32 = 10; // Outer Scope

    {
        println!("The Inner scope can see the outer var_a of {}", var_a);

        let var_a: f32 = 20.223; // Inner Scope
        println!(
            "But in can 'Shadow' it with with it's own version of {}",
            var_a
        );
    }

    println!("See, var_a for the outer scope is still {}", var_a);

    // CONSTATS
    println!("API Code {}", API_CODE);

    let circle_pi = std::f32::consts::PI;
    println!("Circle PI {}", circle_pi);

    // STATIC
    unsafe {
        MY_STATIC_VARIABLE = 10;
        println!("{}", MY_STATIC_VARIABLE);
    }
}
// Global variables are EVIL
// CONSTATS are recomendeds
