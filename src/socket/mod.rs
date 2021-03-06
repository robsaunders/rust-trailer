pub mod binance;
pub use self::binance::BinanceWS;

// pub trait SocketConnection {
//     fn new();
//     fn connect();
//     fn price_change();
// }

#[derive(Debug)]
pub enum Event {
    price_change(String, f64, f64),
}
