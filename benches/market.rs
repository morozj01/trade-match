use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trade_match::matching_engine::market::*;

fn benchmark_add_limit_bid(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 100,000 orders
    for i in 0..100_000 {
        market.add_limit_bid(100.0 + i as f32, 10.0).unwrap();
    }

    c.bench_function("add_limit_bid", |b| {
        b.iter(|| {
            market
                .add_limit_bid(black_box(105.0), black_box(10.0))
                .unwrap();
        })
    });
}

fn benchmark_add_limit_ask(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 100,000 orders
    for i in 0..100_000 {
        market.add_limit_ask(100.0 - i as f32, 10.0).unwrap();
    }

    c.bench_function("add_limit_ask", |b| {
        b.iter(|| {
            market
                .add_limit_ask(black_box(95.0), black_box(10.0))
                .unwrap();
        })
    });
}

fn benchmark_cancel_order(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 100,000 orders and store the IDs
    let mut order_ids = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        let id = market.add_limit_bid(100.0 + i as f32, 10.0).unwrap();
        order_ids.push(id);
    }

    c.bench_function("cancel_order", |b| {
        b.iter(|| {
            market.cancel_limit_order(black_box(order_ids[50000])); // Cancel the 50,000th order
        })
    });
}

fn benchmark_execute_limit_bid(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 100,000 ask orders
    for i in 0..100_000 {
        market.add_limit_ask(100.0 - i as f32, 10.0).unwrap();
    }

    c.bench_function("execute_limit_bid", |b| {
        b.iter(|| {
            market
                .add_limit_bid(black_box(101.0), black_box(1000.0))
                .unwrap(); // This should execute against existing asks
        })
    });
}

fn benchmark_execute_limit_ask(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 100,000 bid orders
    for i in 0..100_000 {
        market.add_limit_bid(100.0 + i as f32, 10.0).unwrap();
    }

    c.bench_function("execute_limit_ask", |b| {
        b.iter(|| {
            market
                .add_limit_ask(black_box(99.0), black_box(1000.0))
                .unwrap(); // This should execute against existing bids
        })
    });
}

criterion_group!(
    benches,
    benchmark_add_limit_bid,
    benchmark_add_limit_ask,
    benchmark_cancel_order,
    benchmark_execute_limit_bid,
    benchmark_execute_limit_ask
);

criterion_main!(benches);
