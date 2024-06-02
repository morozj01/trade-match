use trade_match::matching_engine::market::*;

fn main() {
    // @TODO Implement TCP/IP request processing

    let market = Market::new("AAPL");
    println!("Created a new market for the symbol {:?}", market.symbol())
}
