mod geo_map;

pub fn some_function() {
    let location = geo_map::get_lahore_location();
    println!("Location is at {} , {}", location.lat, location.long);
}

// Lib project
// - Not runnable by itself
// Modular self-contained piece of code
// Callable from some other code
// Hides complexity that the caller shouldn't know about
// Share code
// - or just organizational purpose
// can reference other "crates"
// # Terminology
// - "Package" means "Development Project"
// - "Crates" means a compiled package
// - - publish
// - "Bin" means a "main.rs" based package
// - "Lib" means a "lib.rs" based package
