#![allow(dead_code)]
use std::collections::HashMap;
use rust_decimal::prelude::*;

// To showcase if an item is a bid or an ask from the user
#[derive(Debug)]
pub enum BidorAsk {
    Bid, 
    Ask
}

// Custom pricing it required since we are tieing Limit with a hashmap and f64 values can give inconsistent values while doing so.
#[derive(Debug)]
struct Limit {
    price: Decimal,
    orders: Vec<Order>
}

// A bucket sitting at a price level, containing a bunch of orders from different people with different sizes.
impl Limit {
    // When making a new limit, we need to specify on what price level this limit is going to hold orders for. 
    fn new(price: Decimal) -> Limit {
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
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    // Return result if mutable reference, because we would be sort them in memory
    // BID (BUY ORDER) => FETCH(ASKS) => sorted by cheapest price
    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));

        limits
    }

    // ASK (SELL ORDER) => FETCH(BIDS) => sorted by highest price
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));

        limits
    }

    // For bids - fetch asks
    // For asks - fetch bids
    pub fn fill_market_order(&mut self, market_order: &mut Order) -> () {
        let limits = match market_order.bid_or_ask {
            BidorAsk::Bid => self.ask_limits(),
            BidorAsk::Ask => self.bid_limits(),
        };
    }

    // limit order - will sit in the orderbooks, 
    // market order - will never sit anywhere and will keep coming in and go to exchange, and get filled by a limit order
    pub fn add_limit_order(&mut self, price: Decimal, order: Order) -> () {

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
    use rust_decimal_macros::dec;

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(500), Order::new(10.0, BidorAsk::Ask));
        orderbook.add_limit_order(dec!(100), Order::new(10.0, BidorAsk::Ask));
        orderbook.add_limit_order(dec!(200), Order::new(10.0, BidorAsk::Ask));
        orderbook.add_limit_order(dec!(300), Order::new(10.0, BidorAsk::Ask));
        orderbook.add_limit_order(dec!(50), Order::new(10.0, BidorAsk::Ask));

        // Fill against the cheapest order
        let mut market_order = Order::new(10.0 ,BidorAsk::Bid);
        orderbook.fill_market_order(&mut market_order);

        let ask_limits = orderbook.ask_limits();
        let matched_limit = ask_limits.get(0).unwrap();//.orders.get(0).unwrap();

        // assert_eq!(matched_limit.price, dec!(100));
        // assert_eq!(market_order.is_filled(), true);

        let matched_order = matched_limit.orders.get(0).unwrap();
        assert_eq!(matched_order.is_filled(), true);

    }
    
    #[test]
    fn limit_total_volume() {
        let price = dec!(10000);
        let mut limit:Limit = Limit::new(price);

        let buy_limit_order_a: Order = Order::new(100.00, BidorAsk::Bid);
        let buy_limit_order_b: Order = Order::new(100.00, BidorAsk::Bid);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        println!("{:?}", limit.total_volume());
        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = dec!(10000);
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
        let price = dec!(10000);
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