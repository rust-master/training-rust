// tuples group together data
// data elements in tuples don't have names
// Tuples are common in functional programming
// why use tuples
// you can use certian abstract things with tuples
// you can use tuples instead of explicit data structures
pub fn tuples() {
    let mut some_tuple = (10, 9.3, "a", "b".to_string(), 'c', (1.1, 1.2));
    println!("Tuple Data: {} , {}", some_tuple.0, some_tuple.1);
    println!("Full Tuple {:?}", some_tuple);

    let some_val = (some_tuple.5).1;
    println!("{}", some_val);
    some_tuple.2 = "z";
    some_tuple.5 .0 = 9.9;

    println!("Changed Tuple: {:?}", some_tuple);

    let some_color = get_some_rgb();
    println!("Green {}", some_color.1);
    let (red, green, blue) = some_color;
    println!("r {}  g {}  b {}", red, green, blue);

    // rgba or argb
    let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255);

    let empty_tuple = ();

    match some_color.2 {
        0..=200 => println!("blah!"),
        _ => (),
    }
}

fn get_some_rgb() -> (u8, u8, u8) {
    (200, 100, 10)
}
