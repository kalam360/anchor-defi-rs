use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
enum EventType {
    MarketData,
    Signal,
    Order,
    Fill
}

#[derive(Debug, Clone)]
struct Event {
    event_type: EventType,
    timestamp: DateTime<Utc>,
    priority: u32
}


fn main(){
    println!("Still in works!")

}