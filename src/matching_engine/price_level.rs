use super::order::Order;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PriceLevel {
    price: f32,
    quantity: f32,
    orders: BTreeMap<u64, Order>,
}

impl PriceLevel {
    pub fn new(price_level: f32) -> Self {
        PriceLevel {
            price: price_level,
            quantity: 0.0,
            orders: BTreeMap::new(),
        }
    }

    pub fn peek_next_order(&mut self) -> Option<&mut Order> {
        match self.orders.first_entry() {
            Some(entry) => Some(entry.into_mut()),
            None => None,
        }
    }

    pub fn peek_last_order(&mut self) -> Option<&mut Order> {
        match self.orders.last_entry() {
            Some(entry) => Some(entry.into_mut()),
            None => None,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.add_quantity(order.quantity());
        self.orders.insert(order.id(), order);
    }

    pub fn remove_next_order(&mut self) {
        let removed_order = self.orders.pop_first();

        match removed_order {
            Some((_, removed_order)) => self.remove_quantity(removed_order.quantity()),
            None => {}
        }
    }

    pub fn cancel_order(&mut self, order_id: u64) {
        let removed = self.orders.remove(&order_id);

        if removed.is_some() {
            self.remove_quantity(removed.unwrap().quantity())
        }
    }

    pub fn quantity(&self) -> f32 {
        self.quantity
    }

    pub fn price(&self) -> f32 {
        self.price
    }

    fn add_quantity(&mut self, quantity: f32) {
        self.quantity += quantity;
    }

    fn remove_quantity(&mut self, quantity: f32) {
        self.quantity -= quantity;
    }
}
