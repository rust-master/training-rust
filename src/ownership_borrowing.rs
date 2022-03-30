// Rust Ownership and Borrowing
// Memory Management
// No garbage collection
// Developer Manage Memory in Code
// Stack vs Heap
// --Benefits-
// Runtime speed fast
// Parallel and Concurrent processing
// SAFETY

fn main() {
    // let var_a = String::from("Howdy");
    // let var_b = var_a;
    // println!("{}", var_a); // move occurs because `var_a` has type `String`, which does not implement the `Copy` trait

    // STACK - LIFO
    // - Fast memory creation and retrieval.... speed, Speed, SPEED!
    // - Memory is automatically recaptured by the program after variables go out of scope
    // - Is the default in Rust
    // Fixed Size variables... Collection cannot be Stack based because they can change their size
    // Strings are collection of u8's that can grow and it cannot be on stack
    // Stack variable is rust known, fixed size

    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    // anything have curly bracket will create a new scope
    if stack_i8 == 3 {
        let inside_scope = 9;
        println!("{}", inside_scope);
    } // memory is recaptured after the scope
    // println!("{}", inside_scope); // compiler error

    // Heap
    // - Flexibility
    // - Memory that can grow in size (Vector, HashMap, String, etc)
    // - Runtime performance cost (speed) to allocate memory
    // - Memory that can live beyond the scope that created it
    // - Memory is automatically recaptured when the last owner goes out of the scope
    let heap_vector: Vec<i8> = Vec::new(); //vec![5,2];
    let heap_string: String = String::from("Howdy");
    let heap_i8: Box<i8> = Box::new(30);

    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);
    // Stack copies are cheap
    // stack_i8 and stack_i8_2 "owns" different memory
    // managing memory on heap is expensive

    let heap_i8_2 = heap_i8; // allocated memory remains intact, Ownership is "moved" to heap_i8_2
    println!("{}", heap_i8); // heap_i8 no longer "owns" any memory, rust check it compile time
    println!("{}", heap_i8_2);
    Every piece of data in memory has an owner
    Can only be a single owner of memory at a time
    heap_i8 "owns" that allocated memory
    Other Programming languages
    - two variables pointing the same allocated heap memory
    Parallel and Concurrency issues such as "race conditions"

    "Borrow" ownership as a "reference"
    let heap_i8_3 = &heap_i8;
    let heap_i8_3 = heap_i8.clone(); // Clone creates a "copy" of the memory
    cloning is relatively expensive to the heap
    heap
    println!("{}", heap_i8);
    println!("{}", heap_i8_3);

    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64);
    println!("In main stack {}", heap_f64);
    // Allocated Memory needs one and only one owner

    let some_string: String = String::from("Howdy"); // String always on the heap
    let some_str: &str = "Partner"; // &str is a pointer... to either stack or heap

    some_procedure(some_string, some_str);
}

fn some_procedure(param_a: &String, param_b: &str) {
    println!("{}{}", param_a, param_b);
}

fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack procedure {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
} // when the procedure ends it make the owner to past one

fn some_procedure() {
    let inside_procedure = 9;
} // memory free up after the end curly bracket

// Eliminates memory issues (null pointer, dangling pointers, data races)
// Eliminates the Garbage Collector

// --Motivation--
// Mastery of Ownership and Borrowing will take time
// Be patient
// Stick with it
