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

impl Limit {
    // When making a new limit, we need to specify on what price level this limit is going to hold orders for. 
    fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new()
        }
    }

    // To add orders into the list
    pub fn add_order(&mut self, order: Order) -> () {
        self.orders.push(order);
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