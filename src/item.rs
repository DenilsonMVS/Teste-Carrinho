
#[derive(PartialEq, Clone)]
pub struct Item {
    name: String,
    price: f64
}


impl Item {

    pub fn new(name: String, price: f64) -> Self {
        return Item {
            name: name,
            price: price
        };
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_price(&self) -> f64 {
        return self.price;
    }
}
