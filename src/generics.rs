// struct Point<T> {
//     x: T,
//     y: T,
// }
// // struct Point<T, U> {
// //     x: T,
// //     y: U,
// // }

// fn sum<T>(p: Point<T>) {
//     println!(p.x + p.y);
// }

// enum SomeEnum<T> {
//     OptionA(T),
//     OptionB(T),
//     OptionC,
// }

// // using constraints
// fn some_func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
//     input_a: T,
//     input_b: T,
// ) -> T {
//     println!("input_a has {:?}", input_a);
//     input_a - input_b
// }

// fn some_func2<T, E>(input_a: T, input_b: T, input_e: E) -> T
// where
//     T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
//     E: std::fmt::Debug,
// {
//     println!("input_a has {:?}", input_a);
//     println!("input_e has {:?}", input_e);
//     input_a - input_b
// }

// Traits
// trait SomeCustomTrait {
//     fn blah_blah(&self, a: &str, b: &str) -> String;
// }

// #[derive(Debug)]
// struct SomeStruct {
//     something: i32,
// }

// impl SomeCustomTrait for SomeStruct {
//     fn blah_blah(&self, a: &str, b: &str) -> String {
//         self.something.to_string() + " - " + a + " - " + b
//     }
// }

// fn do_this<T>(some_var: &T) -> String
// where
//     T: SomeCustomTrait + std::fmt::Debug,
// {
//     //Some complex logic
//     println!("{:?}", some_var);
//     some_var.blah_blah("first", "second");
// }

struct UbuntuStruct<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    ubuntu_t: T,
    ubuntu_u: U,
}

impl<T, U> UbuntuStruct<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn log_something(&self) {
        println!("{:?} {:?}", self.ubuntu_t, self.ubuntu_u)
    }
}

fn main() {
    // let a = Point { x: 90., y: 50 };
    // println!("{:?}", a.x);

    // let b = Point { x: 10, y: 20. };
    // println!("{}", b.y);

    // sum(Point { x: 90., y: 50 });
    // println!("Sum = {}", sum(3.3, 2.3));
    // println!("Sum = {}", sum(2.1,4));
    // println!("Sum = {}", sum(8,6.4));

    // let some_data = SomeEnum::OptionA(34, 2);

    // match some_data {
    //     SomeEnum::OptionA(a) => println!("OptionA contain {}", a),
    //     SomeEnum::OptionB(b) => println!("OptionB contain {}", b),
    //     SomeEnum::OptionC => println!("OptionC...."),
    // }

    // let some_data2 = SomeEnum::OptionB('b');
    // let some_data3 = SomeEnum::OptionA(vec![1, 3, 4]);

    // let a = some_func(4, 5);
    // println!("{}", a);

    // let test = SomeStruct { something: 1000 };
    // let result = do_this(&test);

    let test = UbuntuStruct {
        ubuntu_t: 4.1,
        ubuntu_u: vec![1, 2, 4, 5],
    };

    test.log_something();
}

// It's about programming to CAPABILITIES of Type via Constraints on Traits
