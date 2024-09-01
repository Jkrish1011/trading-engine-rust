mod matching_engine;

use matching_engine::orderbook::{BidorAsk, Order, OrderBook};
use matching_engine::engine::{MatchingEngine, TradingPair};
 
fn main() {
    let buy_order_1: Order = Order::new(5.5, BidorAsk::Bid);
    let buy_order_2: Order = Order::new(5.5, BidorAsk::Bid);
    
    let mut orderbook: OrderBook = OrderBook::new();
    orderbook.add_order(5.5, buy_order_1);
    orderbook.add_order(5.5, buy_order_2);

    let sell_order = Order::new(6.5, BidorAsk::Ask);
    orderbook.add_order(20.0, sell_order);

    // println!("{:?}", orderbook);
    let mut engine: MatchingEngine = MatchingEngine::new();
    let pair: TradingPair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());
    let buy_order: Order = Order::new(6.5, BidorAsk::Bid);
    let eth_pair: TradingPair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
