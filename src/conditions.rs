pub fn conditions() {
    let some_bool = true;
    let some_int = 10;
    let some_int2 = 200;

    // if !(some_bool == true) {
    //  if (some_bool == true) {
    // if !some_bool {
    if some_bool == true || (some_int > 100 && some_int2 == 200) {
        // paranthise make the differences
        println!("Hit If branch");
    } else if some_int == 300 {
        println!("Hit Else If branch");
    } else {
        println!("Hit Else branch");
    }

    let var_from_inline = if some_int == 9 {
        300
    } else if some_int == -9 {
        println!("Hit Else If branch");
        900
    } else {
        400
    }; // dont need semicolon after if return statement -- Similar to mini function
       // all branches if, else if and else will return the same type of data
    println!("var_from_inline: {}", var_from_inline);

    // match
    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }

    match some_int {
        0 => println!("Hit 0 branch"),
        1..=100 => println!("Between 1 and 100 branch"),
        _ => println!("Else branch"),
    } // it helpful in the enumuration

    let result = match some_int {
        0 => println!("Hit 0 branch"),
        1 | 100 => println!("Between 1 and 100 branch"),
        _ => println!("Else branch"),
    }; // it helpful in the enumuration
}
