use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trade_match::matching_engine::market::*;

fn benchmark_add_limit_bids_many_levels(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // 100K bids each at a different price level
    c.bench_function("add_limit_bids_many_levels", |b| {
        b.iter(|| {
            for i in 0..100_000 {
                market.add_limit_bid(100.0 + i as f32, 10.0).unwrap();
            }
        })
    });
}

fn benchmark_add_limit_asks_many_levels(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // 100K asks each at a different price level
    c.bench_function("add_limit_asks_many_levels", |b| {
        b.iter(|| {
            for i in 0..100_000 {
                market.add_limit_ask(100.0 + i as f32, 10.0).unwrap();
            }
        })
    });
}

fn benchmark_add_limit_bids_single_level(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // 100K bids at a single price level
    c.bench_function("add_limit_bids_single_level", |b| {
        b.iter(|| {
            for _i in 0..100_000 {
                market.add_limit_bid(100.0, 10.0).unwrap();
            }
        })
    });
}

fn benchmark_add_limit_asks_single_level(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // 100K asks at a single price level
    c.bench_function("add_limit_asks_single_level", |b| {
        b.iter(|| {
            for _i in 0..100_000 {
                market.add_limit_ask(100.0, 10.0).unwrap();
            }
        })
    });
}

fn benchmark_cancel_order(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 1,000,000 orders
    for _i in 0..1_000_000 {
        market.add_limit_bid(100.0, 10.0).unwrap();
    }

    let mut counter = 1;

    c.bench_function("cancel_order", |b| {
        b.iter(|| {
            market.cancel_limit_order(black_box(counter));
            counter += 1;
        })
    });
}

fn benchmark_execute_limit_bid(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 1,000,000 ask orders
    for i in 1..1_000_000 {
        market.add_limit_ask(100.0 + i as f32, 100_000.0).unwrap();
    }

    c.bench_function("execute_limit_bid", |b| {
        b.iter(|| {
            // This should execute against existing asks
            market
                .add_limit_bid(black_box(120_000.0), black_box(1000.0))
                .unwrap();
        })
    });
}

fn benchmark_execute_limit_ask(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 1,000,000 bid orders
    for i in 1..1_000_000 {
        market.add_limit_bid(100.0 + i as f32, 100_000.0).unwrap();
    }

    c.bench_function("execute_limit_ask", |b| {
        b.iter(|| {
            // This should execute against existing bids
            market
                .add_limit_ask(black_box(99.0), black_box(1000.0))
                .unwrap();
        })
    });
}

fn benchmark_add_market_bid(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 1,000,000 ask orders
    for i in 1..1_000_000 {
        market.add_limit_ask(100.0 + i as f32, 100_000.0).unwrap();
    }

    c.bench_function("add_market_bid", |b| {
        b.iter(|| {
            // This should execute against existing asks
            market.add_market_bid(black_box(1000.0));
        })
    });
}

fn benchmark_add_market_ask(c: &mut Criterion) {
    let mut market = Market::new("BTCUSD");

    // Prepopulate the market with 1,000,000 bid orders
    for i in 0..1_000_000 {
        market.add_limit_bid(100.0 + i as f32, 100_000.0).unwrap();
    }

    c.bench_function("add_market_ask", |b| {
        b.iter(|| {
            // This should execute against existing bids
            market.add_market_ask(black_box(1000.0));
        })
    });
}

criterion_group!(
    benches,
    benchmark_add_limit_bids_many_levels,
    benchmark_add_limit_asks_many_levels,
    benchmark_add_limit_bids_single_level,
    benchmark_add_limit_asks_single_level,
    benchmark_cancel_order,
    benchmark_execute_limit_bid,
    benchmark_execute_limit_ask,
    benchmark_add_market_bid,
    benchmark_add_market_ask
);

criterion_main!(benches);
