use trade_match::matching_engine::market::*;

#[test]
fn test_create_market() {
    let market = Market::new("BTCUSD");
    assert_eq!(market.symbol(), "BTCUSD");
}

#[test]
fn test_initial_best_bid_and_best_ask() {
    let market = Market::new("BTCUSD");
    assert_eq!(market.best_bid(), f32::NEG_INFINITY);
    assert_eq!(market.best_ask(), f32::INFINITY);
}

#[test]
fn test_add_multiple_limit_bids() {
    let mut market = Market::new("BTCUSD");
    market.add_limit_bid(100.0, 10.0).unwrap();
    market.add_limit_bid(101.0, 10.0).unwrap();
    assert_eq!(market.best_bid(), 101.0);
}

#[test]
fn test_add_multiple_limit_asks() {
    let mut market = Market::new("BTCUSD");
    market.add_limit_ask(100.0, 10.0).unwrap();
    market.add_limit_ask(99.0, 10.0).unwrap();
    assert_eq!(market.best_ask(), 99.0);
}

#[test]
fn test_execute_limit_ask() {
    let mut market = Market::new("BTCUSD");

    market.add_limit_bid(101.0, 5.0).unwrap();
    market.add_limit_bid(102.0, 5.0).unwrap();
    market.add_limit_bid(103.0, 5.0).unwrap();
    market.add_limit_bid(104.0, 5.0).unwrap();

    assert_eq!(market.best_bid(), 104.0);

    // marketable order
    market.add_limit_ask(100.0, 10.0).unwrap();

    assert_eq!(market.best_bid(), 102.0);
}

#[test]
fn test_execute_limit_bid() {
    let mut market = Market::new("BTCUSD");

    market.add_limit_ask(100.0, 5.0).unwrap();
    market.add_limit_ask(99.0, 5.0).unwrap();
    market.add_limit_ask(98.0, 5.0).unwrap();
    market.add_limit_ask(97.0, 5.0).unwrap();

    assert_eq!(market.best_ask(), 97.0);

    // marketable order
    market.add_limit_bid(101.0, 10.0).unwrap();

    assert_eq!(market.best_ask(), 99.0);
}

#[test]
fn test_add_market_bid() {
    let mut market = Market::new("BTCUSD");

    market.add_limit_ask(99.0, 5.0).unwrap();
    market.add_limit_ask(100.0, 5.0).unwrap();
    market.add_limit_ask(101.0, 5.0).unwrap();
    market.add_limit_ask(102.0, 5.0).unwrap();
    market.add_limit_ask(103.0, 5.0).unwrap();

    assert_eq!(market.best_ask(), 99.0);

    market.add_market_bid(14.99);

    assert_eq!(market.best_ask(), 101.0);

    market.add_market_bid(5.00);

    assert_eq!(market.best_ask(), 102.0);
    assert_eq!(market.best_bid(), f32::NEG_INFINITY);
}

#[test]
fn test_add_market_ask() {
    let mut market = Market::new("BTCUSD");

    market.add_limit_bid(10.0, 5.0).unwrap();
    market.add_limit_bid(9.0, 5.0).unwrap();
    market.add_limit_bid(8.0, 5.0).unwrap();
    market.add_limit_bid(7.0, 5.0).unwrap();
    market.add_limit_bid(6.0, 5.0).unwrap();

    assert_eq!(market.best_bid(), 10.0);

    market.add_market_ask(14.99);

    assert_eq!(market.best_bid(), 8.0);

    market.add_market_ask(5.0);

    assert_eq!(market.best_bid(), 7.0);
    assert_eq!(market.best_ask(), f32::INFINITY);
}

#[test]
fn test_cancel_nonexistent_order() {
    let mut market = Market::new("BTCUSD");
    assert!(!market.cancel_limit_order(1));
}

#[test]
fn test_cancel_existing_order() {
    let mut market = Market::new("BTCUSD");
    let order_id = market.add_limit_bid(100.0, 10.0).unwrap();
    assert!(market.cancel_limit_order(order_id));
}

#[test]
fn test_cancel_order_updates_best_bid() {
    let mut market = Market::new("BTCUSD");
    let order_id = market.add_limit_bid(100.0, 10.0).unwrap();
    market.add_limit_bid(101.0, 10.0).unwrap();
    market.cancel_limit_order(order_id);
    assert_eq!(market.best_bid(), 101.0);
}

#[test]
fn test_cancel_order_updates_best_ask() {
    let mut market = Market::new("BTCUSD");
    let order_id = market.add_limit_ask(100.0, 10.0).unwrap();
    market.add_limit_ask(99.0, 10.0).unwrap();
    market.cancel_limit_order(order_id);
    assert_eq!(market.best_ask(), 99.0);
}

#[test]
fn test_partially_filled_limit_bid() {
    let mut market = Market::new("BTCUSD");
    market.add_limit_ask(100.0, 5.0).unwrap();
    let order_id = market.add_limit_bid(100.0, 10.0).unwrap();
    assert_eq!(market.best_bid(), 100.0);
    // Check if the partially filled order remains
    assert!(market.order_exists(order_id));
}

#[test]
fn test_partially_filled_limit_ask() {
    let mut market = Market::new("BTCUSD");
    market.add_limit_bid(100.0, 5.0).unwrap();
    let order_id = market.add_limit_ask(100.0, 10.0).unwrap();
    assert_eq!(market.best_ask(), 100.0);
    // Check if the partially filled order remains
    assert!(market.order_exists(order_id));
}
