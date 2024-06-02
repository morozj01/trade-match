use std::hash::{Hash, Hasher};

// Ascending by price
#[derive(Debug)]
pub struct PriceLevelKeyAsk(f32);

impl PriceLevelKeyAsk {
    pub fn new(price: f32) -> Self {
        PriceLevelKeyAsk(price)
    }

    pub fn get_price(&self) -> f32 {
        self.0
    }
}

impl PartialOrd for PriceLevelKeyAsk {
    fn partial_cmp(&self, other: &PriceLevelKeyAsk) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for PriceLevelKeyAsk {
    fn cmp(&self, other: &PriceLevelKeyAsk) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialEq for PriceLevelKeyAsk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PriceLevelKeyAsk {}

impl Hash for PriceLevelKeyAsk {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.to_bits());
    }
}

// Descending by price
#[derive(Debug)]
pub struct PriceLevelKeyBid(f32);

impl PriceLevelKeyBid {
    pub fn new(price: f32) -> Self {
        PriceLevelKeyBid(price)
    }

    pub fn get_price(&self) -> f32 {
        self.0
    }
}

impl PartialOrd for PriceLevelKeyBid {
    fn partial_cmp(&self, other: &PriceLevelKeyBid) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for PriceLevelKeyBid {
    fn cmp(&self, other: &PriceLevelKeyBid) -> std::cmp::Ordering {
        other.0.total_cmp(&self.0)
    }
}

impl PartialEq for PriceLevelKeyBid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PriceLevelKeyBid {}

impl Hash for PriceLevelKeyBid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.to_bits());
    }
}
