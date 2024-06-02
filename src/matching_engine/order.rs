#[derive(Debug)]
pub enum OrderType {
    Limit,
}

#[derive(Debug)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    id: u64,
    quantity: f32,
}

impl Order {
    pub fn new(id: u64, quantity: f32) -> Self {
        Order {
            id: id,
            quantity: quantity,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn remove_quantity(&mut self, quantity: f32) {
        self.quantity -= quantity;
    }

    pub fn quantity(&self) -> f32 {
        self.quantity
    }
}
