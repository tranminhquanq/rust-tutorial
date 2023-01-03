pub struct Animal {
    name: String,
    age: u32,
}

impl Animal {
    fn init(&self, name: String, age: u32) -> Self {
        return Self {
            name,
            age,
        }
    }
}