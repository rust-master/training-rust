// Fuctions and Procedures in Rust

pub fn some_string_procedure(param: String) {
    println!("{}", param);
}

pub fn some_str_procedure(param_a: &str) {
    println!("{}", param_a);
}

pub fn some_procedure(param_a: f32, param_b: i8) {
    println!("Procedure: {} {}", param_a, param_b);
}

pub fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("Here is some function");

    // let returned_var = 10.1 * param_a + param_b; // get compiler error can not add integer and float
    // let returned_var = 10.1 * param_a + param_b as f32;

    if param_a < 100. {
        let return_var = 10.1 * param_a + param_b as f32;

        return_var
    } else {
        -1.
    }

    // 10.1 // No simicolon means return statement
    // 10 as f32
    // 10f32
    // 10_f32
    // 10.
}

// use snake case convention in rust -- Recommended
