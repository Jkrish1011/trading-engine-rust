#![allow(dead_code)]
use std::collections::HashMap;

// To showcase if an item is a bid or an ask from the user
#[derive(Debug)]
pub enum BidorAsk {
    Bid, 
    Ask
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

// Like constructor for Price.
impl Price {
    pub fn new(price: f64) -> Self {
        let scalar: u64 = 10000;
        let integral: u64 = price as u64; // Just the integral part of the price
        let fractional: u64 = (((price % 1.0) * scalar as f64)) as u64;
        Price {
            scalar,
            integral,
            fractional
        }
    }
}

// Custom pricing it required since we are tieing Limit with a hashmap and f64 values can give inconsistent values while doing so.
#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>
}

// A bucket sitting at a price level, containing a bunch of orders from different people with different sizes.
impl Limit {
    // When making a new limit, we need to specify on what price level this limit is going to hold orders for. 
    fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new()
        }
    }

    fn total_volume(&self) -> f64 {
        // Works similar to the reduce function in javascript.
        // We have to unwrap this because it will be returning an Option. In case of error, the program will panic!
        self.orders.iter().map(|order| order.size).reduce(|a, b| a + b).unwrap()
    }

    // To add orders into the list
    pub fn add_order(&mut self, order: Order) -> () {
        self.orders.push(order);
    }

    // To fill the order
    fn fill_order(&mut self, market_order: &mut Order) -> () {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                },
            }

            if market_order.is_filled() {
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidorAsk
}

impl Order {
    // if the variables have same name as the key, it can be mentioned like this. also keys can be used as well.
    pub fn new(size: f64, bid_or_ask: BidorAsk) -> Order { // Either Self or Order can be used as return type
        Order {
            size: size,
            bid_or_ask: bid_or_ask
        }
    }

    // Check if the Order is filled
    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    // TODO: Sort in the highest to lowest order
    pub fn ask_limits(&mut self) => Vec<&mut Limit> {
        self.asks.values_mut().collect::<Vec<&mut Limit>>()
    }

    // TODO: Sort in the lowest to highest order
    pub fn bid_limits(&mut self) => Vec<&mut Limit> {
        self.bids.values_mut().collect::<Vec<&mut Limit>>()
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) -> () {
        match market_order.bid_or_ask {
            BidorAsk::Bid => {
                for limit_order in self.ask_limits() {
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            },
            BidorAsk::Ask => {

            },
        }
    }

    // limit order - will sit in the orderbooks, 
    // market order - will never sit anywhere and will keep coming in and go to exchange, and get filled by a limit order
    pub fn add_order(&mut self, price: f64, order: Order) -> () {
        let price: Price = Price::new(price);

        // Check if order exists in a price limit, and then append or make a new one accordingly
        match order.bid_or_ask {
            BidorAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    println!("info::from-ask-limit-already-exists");
                    limit.add_order(order);
                }
                None => {
                    println!("info::from-ask-to-create-new-limit");
                    let mut limit: Limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidorAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    println!("info::from-ask-limit-already-exists");
                    limit.add_order(order);
                }
                None => {
                    println!("info::from-ask-to-create-new-limit");
                    let mut limit: Limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_total_volume() {
        let price: Price = Price::new(1000.0);
        let mut limit:Limit = Limit::new(price);

        let buy_limit_order_a: Order = Order::new(100.0, BidorAsk::Bid);
        let buy_limit_order_b: Order = Order::new(100.0, BidorAsk::Bid);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        println!("{:?}", limit.total_volume());
        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_multi000_fill() {
        let price: Price = Price::new(1000.0);
        let mut limit:Limit = Limit::new(price);

        let buy_limit_order_a: Order = Order::new(100.0, BidorAsk::Bid);
        let buy_limit_order_b: Order = Order::new(100.0, BidorAsk::Bid);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order: Order = Order::new(199.0, BidorAsk::Ask);
        limit.fill_order(&mut market_sell_order);
        
        
        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        println!("{:?}", limit);

    }

    #[test]
    fn limit_order_single_fill() {
        let price: Price = Price::new(1000.0);
        let mut limit:Limit = Limit::new(price);

        let buy_limit_order: Order = Order::new(100.0, BidorAsk::Bid);
        limit.add_order(buy_limit_order);

        let mut market_sell_order: Order = Order::new(99.0, BidorAsk::Ask);
        limit.fill_order(&mut market_sell_order);
        println!("{:?}", limit);
        
        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);

    }
}