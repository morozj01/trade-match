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

    pub fn add_limit_bid(&mut self, price: f32, mut quantity: f32) -> Result<u64, &str> {
        if !Market::check_precision(price) {
            return Err("Price cannot have more then 2 decimal places");
        }

        // marketable order
        if price >= self.lowest_ask {
            quantity = self.execute_limit_bid(price, quantity);
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
            return Err("Price cannot have more then 2 decimal places");
        }

        // marketable order
        if price <= self.highest_bid {
            quantity = self.execute_limit_ask(price, quantity);
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

    fn execute_limit_bid(&mut self, price: f32, mut quantity: f32) -> f32 {
        let range_start = PriceLevelKeyAsk::new(self.lowest_ask);
        let range_end = PriceLevelKeyAsk::new(price + 0.1);

        let mut final_reached = self.lowest_ask;
        let mut final_has_quantity = true;

        for (_, price_level) in self.ask_levels.range_mut(range_start..range_end) {
            while quantity > 0.0 && price_level.peek_next_order().is_some() {
                let next_order = price_level.peek_next_order().unwrap();
                match next_order.quantity() <= quantity {
                    true => {
                        quantity -= next_order.quantity();
                        self.orders.remove(&next_order.id());
                        price_level.remove_next_order();
                    }
                    false => {
                        next_order.remove_quantity(quantity);
                        quantity = 0.0;
                    }
                }
            }

            final_reached = price_level.price();
            final_has_quantity = price_level.quantity() > 0.0;
        }

        if final_has_quantity {
            self.lowest_ask = final_reached;
        } else {
            self.reset_best_ask(Some(final_reached));
        }

        quantity
    }

    fn execute_limit_ask(&mut self, price: f32, mut quantity: f32) -> f32 {
        let range_start = PriceLevelKeyBid::new(self.highest_bid);
        let range_end = PriceLevelKeyBid::new(price - 0.1);

        let mut final_reached = self.lowest_ask;
        let mut final_has_quantity = true;

        for (_, price_level) in self.bid_levels.range_mut(range_start..range_end) {
            while quantity > 0.0 && price_level.peek_next_order().is_some() {
                let next_order = price_level.peek_next_order().unwrap();
                match next_order.quantity() <= quantity {
                    true => {
                        quantity -= next_order.quantity();
                        self.orders.remove(&next_order.id());
                        price_level.remove_next_order();
                    }
                    false => {
                        next_order.remove_quantity(quantity);
                        quantity = 0.0;
                    }
                }
            }

            final_reached = price_level.price();
            final_has_quantity = price_level.quantity() > 0.0;
        }

        if final_has_quantity {
            self.highest_bid = final_reached;
        } else {
            self.reset_best_bid(Some(final_reached));
        }

        quantity
    }

    fn reset_best_bid(&mut self, mut cursor: Option<f32>) {
        if cursor.is_none() {
            cursor = Some(self.highest_bid);
        }

        let mut cursor = self
            .bid_levels
            .lower_bound(Included(&&PriceLevelKeyBid::new(cursor.unwrap())));

        let mut final_has_quantity = false;

        while cursor.peek_next().is_some() {
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

        while cursor.peek_next().is_some() {
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
        (float * 100.0).fract() == 0.0
    }
}
