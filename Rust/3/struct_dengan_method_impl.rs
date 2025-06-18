struct Car {
    brand: String,
    year: u16,
}

impl Car {
    fn info(&self) {
        println!("{} ({})", self.brand, self.year);
    }
}

fn main() {
    let car = Car {
        brand: String::from("Toyota"),
        year: 2020,
    };
    car.info();
}