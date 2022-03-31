pub struct Location {
    pub lat: f32,
    pub long: f32,
}

pub fn get_lahore_location() -> Location {
    Location {
        lat: 31.520,
        long: 74.358,
    }
}
