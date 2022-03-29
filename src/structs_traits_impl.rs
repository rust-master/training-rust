// struct represent complex data types that you define
// structs are like "objects", but have difference
// rust does'nt have inheritance
// rust does have methods
// rust has "Traits" -- Similar to poymorphism for object oriented
// Derived traits can be done using macros

pub struct Records {
    pub call_count: i64,
    pub data: String,
    pub timestamp: i32,
}

// in other programming language we can add method to struct
// but in struct it define outside the impl (Implementation)
// Capital "Self" means type and lower "self" means actual data
// without self param fn use with four colons :: and
// with self param fn use with dot .

impl Records {
    pub fn new(param_a: i32) -> Self {
        Self {
            call_count: 0,
            data: "NewData".to_string(),
            timestamp: param_a,
        }
    }

    pub fn time_check(&mut self, now: i32) -> i32 {
        self.call_count += 1;
        self.timestamp - now
    }
}
