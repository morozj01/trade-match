use super::order::*;
use super::price_level::*;
use super::price_level_key::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Bound::Included;

#[derive(Debug)]
pub struct Market<'a> {
    symbol: &'a str,
    total_orders: u64,
    lowest_ask: f32,
    highest_bid: f32,
    ask_levels: BTreeMap<PriceLevelKeyAsk, PriceLevel>,
    bid_levels: BTreeMap<PriceLevelKeyBid, PriceLevel>,
    orders: HashMap<u64, (OrderSide, f32)>,
}

impl<'a> Market<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Market {
            symbol,
            total_orders: 0,
            lowest_ask: f32::INFINITY,
            highest_bid: f32::NEG_INFINITY,
            ask_levels: BTreeMap::new(),
            bid_levels: BTreeMap::new(),
            orders: HashMap::new(),
        }
    }

    pub fn symbol(&self) -> &str {
        self.symbol
    }

    pub fn best_bid(&self) -> f32 {
        self.highest_bid
    }

    pub fn best_ask(&self) -> f32 {
        self.lowest_ask
    }

    pub fn order_exists(&self, order_id: u64) -> bool {
        self.orders.contains_key(&order_id)
    }

    pub fn add_market_bid(&mut self, quantity: f32) -> (bool, f32) {
        if self.lowest_ask == f32::INFINITY {
            return (false, quantity);
        }

        let quantity_remaining = self.execute_bid(None, quantity);

        if quantity_remaining == 0.0 {
            return (true, 0.0);
        } else {
            return (false, quantity_remaining);
        }
    }

    pub fn add_market_ask(&mut self, quantity: f32) -> (bool, f32) {
        if self.highest_bid == f32::NEG_INFINITY {
            return (false, quantity);
        }

        let quantity_remaining = self.execute_ask(None, quantity);

        if quantity_remaining == 0.0 {
            return (true, 0.0);
        } else {
            return (false, quantity_remaining);
        }
    }

    pub fn add_limit_bid(&mut self, price: f32, mut quantity: f32) -> Result<u64, &str> {
        if !Market::check_precision(price) {
            return Err("Price must be positive and cannot have more then 2 decimal places");
        }

        // marketable order
        if price >= self.lowest_ask {
            quantity = self.execute_bid(Some(price), quantity);
        }

        if quantity <= 0.0 {
            return Ok(0);
        } else {
            let id = self.increment_total_orders();

            let order = Order::new(id, quantity);

            self.highest_bid = f32::max(price, self.highest_bid);

            let level_key = PriceLevelKeyBid::new(price);

            match self.bid_levels.get_mut(&level_key) {
                Some(price_level) => {
                    price_level.add_order(order);
                }
                None => {
                    let mut new_level = PriceLevel::new(price);
                    new_level.add_order(order);
                    self.bid_levels.insert(level_key, new_level);
                }
            }

            self.orders.insert(id, (OrderSide::Bid, price));
            Ok(id)
        }
    }

    pub fn add_limit_ask(&mut self, price: f32, mut quantity: f32) -> Result<u64, &str> {
        if !Market::check_precision(price) {
            return Err("Price must be positive and cannot have more then 2 decimal places");
        }

        // marketable order
        if price <= self.highest_bid {
            quantity = self.execute_ask(Some(price), quantity);
        }

        if quantity <= 0.0 {
            return Ok(0);
        } else {
            let id = self.increment_total_orders();

            let order = Order::new(id, quantity);

            self.lowest_ask = f32::min(price, self.lowest_ask);

            let level_key = PriceLevelKeyAsk::new(price);

            match self.ask_levels.get_mut(&level_key) {
                Some(price_level) => {
                    price_level.add_order(order);
                }
                None => {
                    let mut new_level = PriceLevel::new(price);
                    new_level.add_order(order);
                    self.ask_levels.insert(level_key, new_level);
                }
            }

            self.orders.insert(id, (OrderSide::Ask, price));
            Ok(id)
        }
    }

    pub fn cancel_limit_order(&mut self, id: u64) -> bool {
        match self.orders.get_mut(&id) {
            Some((side, price)) => match side {
                OrderSide::Ask => {
                    let level = self.ask_levels.get_mut(&PriceLevelKeyAsk::new(*price));
                    level.unwrap().cancel_order(id);

                    if *price == self.lowest_ask {
                        self.reset_best_ask(None);
                    }

                    self.orders.remove(&id);

                    true
                }
                OrderSide::Bid => {
                    let level = self.bid_levels.get_mut(&PriceLevelKeyBid::new(*price));
                    level.unwrap().cancel_order(id);

                    if *price == self.highest_bid {
                        self.reset_best_bid(None);
                    }

                    self.orders.remove(&id);

                    true
                }
            },
            None => false,
        }
    }

    fn execute_ask(&mut self, price: Option<f32>, mut quantity: f32) -> f32 {
        let price = price.unwrap_or_else(|| f32::NEG_INFINITY);

        let mut cursor = self
            .bid_levels
            .lower_bound_mut(Included(&PriceLevelKeyBid::new(self.highest_bid)));

        // iterate over price levels
        while quantity > 0.0 && cursor.value().is_some() {
            let level = cursor.value_mut().unwrap();

            // iterate over orders within a single price level
            while quantity > 0.0 && level.peek_next_order().is_some() {
                let next_order = level.peek_next_order().unwrap();

                match next_order.quantity() <= quantity {
                    true => {
                        quantity -= next_order.quantity();
                        self.orders.remove(&next_order.id());
                        level.remove_next_order();
                    }
                    false => {
                        next_order.remove_quantity(quantity);
                        quantity = 0.0;
                    }
                }
            }

            if quantity > 0.0 && level.price() >= price {
                cursor.move_next();
            } else {
                break;
            }
        }

        if quantity > 0.0 {
            self.highest_bid = f32::NEG_INFINITY;
            return quantity;
        } else {
            let cursor_price = cursor.value().unwrap().price();
            self.reset_best_bid(Some(cursor_price));
            return 0.0;
        }
    }

    fn execute_bid(&mut self, price: Option<f32>, mut quantity: f32) -> f32 {
        let price = price.unwrap_or_else(|| f32::INFINITY);

        let mut cursor = self
            .ask_levels
            .lower_bound_mut(Included(&PriceLevelKeyAsk::new(self.lowest_ask)));

        // iterate over price levels
        while quantity > 0.0 && cursor.value().is_some() {
            let level = cursor.value_mut().unwrap();

            // iterate over orders within a single price level
            while quantity > 0.0 && level.peek_next_order().is_some() {
                let next_order = level.peek_next_order().unwrap();

                match next_order.quantity() <= quantity {
                    true => {
                        quantity -= next_order.quantity();
                        self.orders.remove(&next_order.id());
                        level.remove_next_order();
                    }
                    false => {
                        next_order.remove_quantity(quantity);
                        quantity = 0.0;
                    }
                }
            }

            if quantity > 0.0 && level.price() <= price {
                cursor.move_next();
            } else {
                break;
            }
        }

        if quantity > 0.0 {
            self.lowest_ask = f32::INFINITY;
            return quantity;
        } else {
            let cursor_price = cursor.value().unwrap().price();
            self.reset_best_ask(Some(cursor_price));
            return 0.0;
        }
    }

    fn reset_best_bid(&mut self, mut cursor: Option<f32>) {
        if cursor.is_none() {
            cursor = Some(self.highest_bid);
        }

        let mut cursor = self
            .bid_levels
            .lower_bound(Included(&&PriceLevelKeyBid::new(cursor.unwrap())));

        let mut final_has_quantity = false;

        while cursor.value().is_some() {
            let level = cursor.value().unwrap();

            if level.quantity() > 0.0 {
                final_has_quantity = true;
                self.highest_bid = level.price();
                break;
            } else {
                cursor.move_next();
            }
        }

        if !final_has_quantity {
            self.highest_bid = f32::NEG_INFINITY;
        }
    }

    fn reset_best_ask(&mut self, mut cursor: Option<f32>) {
        if cursor.is_none() {
            cursor = Some(self.lowest_ask);
        }

        let mut cursor = self
            .ask_levels
            .lower_bound(Included(&&PriceLevelKeyAsk::new(cursor.unwrap())));

        let mut final_has_quantity = false;

        while cursor.value().is_some() {
            let level = cursor.value().unwrap();

            if level.quantity() > 0.0 {
                final_has_quantity = true;
                self.lowest_ask = level.price();
                break;
            } else {
                cursor.move_next();
            }
        }

        if !final_has_quantity {
            self.lowest_ask = f32::INFINITY;
        }
    }

    fn increment_total_orders(&mut self) -> u64 {
        self.total_orders += 1;
        self.total_orders
    }

    fn check_precision(float: f32) -> bool {
        (float * 100.0).fract() == 0.0 && float > 0.0
    }
}
