pub fn primitve() {
    let some_data: bool = true;

    if some_data {
        println!("true");
    } else {
        println!("false");
    }

    let some_data: i8 = -128; // -128 to 127
    println!("Max i8: {}", i8::MAX);
    println!("Min i8: {}", i8::MIN);

    let some_data: u8 = 10; // 0 to 255
    let some_other_data = 255 + some_data; // ERROR: to big - overflow
    let some_other_data2 = 1 - some_data; // ERROR: to small - underflow

    let some_data: i128 = 10;
    println!("Max i128: {}", i128::MAX);
    println!("Min i128: {}", i128::MIN);

    let _some_data: i32 = 10;
    println!("Max i32: {}", i32::MAX);
    println!("Min i32: {}", i32::MIN);

    // some computer are 32 bit and 64 bit

    let some_isize: isize = 10; // depends on whether the computer is 32 or 64 bits
    let some_usize: usize = 10; // depends on whether the computer is 32 or 64 bits
    println!("Max isize: {}", isize::MAX);
    println!("Min isize: {}", isize::MIN);

    let some_data: f32 = 10.; // Search the Float to undertand - Incomplete yet
    println!("Max f32: {}", f32::MAX);

    let some_char: char = 'a';
}
