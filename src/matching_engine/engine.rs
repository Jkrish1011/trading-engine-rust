use std::collections::HashMap;
use rust_decimal::prelude::*;

use super::orderbook::{Order, OrderBook};


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String, // will be owned
    quote: String, 
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

// Will hold multiple orderbooks
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) -> () {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("info::opening-new-orderbook-for-market::{:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String>{
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("info::placed-limit-order-price-level::{}", price);
                Ok(())
            }
            None => {
                Err(format!("error::the-orderbook-for-given-tradingpair-{}-does-not-exsit", pair.to_string()))
            }
        }
    }
}