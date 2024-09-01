// To showcase if an item is a bid or an ask from the user
#[derive(Debug)]
enum BidorAsk {
    Bid, 
    Ask
}

#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,

}

// Like constructor for Price.
impl Price {
    fn new(price: f64) -> Self {
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
    fn new(price: f64) -> Limit {
        Limit {
            price: Price::new(price),
            orders: Vec::new()
        }
    }

    // To add orders into the list
    fn add_order(&mut self, order: Order) -> () {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidorAsk
}

impl Order {
    // if the variables have same name as the key, it can be mentioned like this. also keys can be used as well.
    fn new(size: f64, bid_or_ask: BidorAsk) -> Order { // Either Self or Order can be used as return type
        Order {
            size: size,
            bid_or_ask: bid_or_ask
        }
    }
}

fn main() {
    let mut limit: Limit = Limit::new(65.0);
    let buy_order: Order = Order::new(5.5, BidorAsk::Bid);
    let sell_order: Order = Order::new(2.45, BidorAsk::Ask);
    limit.add_order(buy_order);
    limit.add_order(sell_order);
    println!("{:?}", limit);
}
