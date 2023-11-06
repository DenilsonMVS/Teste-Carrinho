
mod item;
mod shopping_cart;

#[cfg(test)]
mod tests {

    use crate::item::Item;
    use crate::shopping_cart::ShoppingCart;

    fn setup() -> ShoppingCart {
        let mut cart = ShoppingCart::new();
        cart.add_item(Item::new("ESM".to_owned(), 65.0));
        cart.add_item(Item::new("GoF".to_owned(), 65.0));
        return cart;
    }

    #[test]
    fn test_add_item() {
        let mut cart = setup();
        let item = Item::new("ABC".to_owned(), 80.5);
        cart.add_item(item.clone());
        assert!(cart.get_item_count() == 3);
        assert!(*cart.get_items().last().unwrap() == item);
    }

    #[test]
    fn test_remove_item() {
        let mut cart = setup();
        cart.remove_item(&Item::new("ESM".to_owned(), 65.0));
        assert!(cart.get_item_count() == 1);
    }

    #[test]
    fn test_get_total_price() {
        let cart = setup();
        assert!(cart.get_total_price() == 130.0);
    }

    #[test]
    fn test_clear_cart() {
        let mut cart = setup();
        cart.clear_cart();
        assert!(cart.get_item_count() == 0);
    }
}
