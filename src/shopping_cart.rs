
use crate::item::Item;

pub struct ShoppingCart {
    items: Vec<Item>
}


impl ShoppingCart {

    pub fn new() -> Self {
        return Self {
            items: Vec::new()
        };
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &Item) {
        self.items.retain(|x| x != item);
    }

    pub fn clear_cart(&mut self) {
        self.items.clear();
    }

    pub fn get_item_count(&self) -> usize {
        return self.items.len();
    }

    pub fn get_total_price(&self) -> f64 {
        return self.items.iter().fold(0.0, |acc, x| acc + x.get_price());
    }

    pub fn get_items(&self) -> &[Item] {
        return &self.items;
    }
}
