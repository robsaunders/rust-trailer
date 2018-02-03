#![allow(dead_code)]

mod trades;
pub use self::trades::*;

#[derive(Debug, Clone, Copy)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub order_type: String,
    pub amount: f64,
    pub price: f64,
}

pub struct CoinAsset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: String,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_show_trades() {
//         assert_eq!(
//             show_trades(),

//         );
//     }
// }

// pub struct TradeHistory {
//     pub balance: f64,
//     pub average_buy_price: f64,
//     pub average_sell_price: f64,
//     pub profit_locked: f64,
// }