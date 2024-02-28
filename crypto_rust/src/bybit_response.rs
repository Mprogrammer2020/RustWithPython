use pyo3::prelude::*;
use bybit::ws::response::{Trade, FutureTicker, Orderbook, OrderbookItem, SpotTicker};
use pyo3::types::*;


pub struct TradeWrapper<'a>(pub &'a Trade<'a>);

impl<'a> ToPyObject for TradeWrapper<'a> {
    fn to_object(&self, py: Python) -> PyObject {
        let trade_dict: &PyDict = PyDict::new(py);
        trade_dict.set_item("T", self.0.T).unwrap();
        trade_dict.set_item("s", self.0.s).unwrap();
        trade_dict.set_item("S", self.0.S).unwrap();
        trade_dict.set_item("v", self.0.v).unwrap();
        trade_dict.set_item("p", self.0.p).unwrap();
        trade_dict.set_item("L", self.0.L.as_deref()).unwrap();
        trade_dict.set_item("i", self.0.i).unwrap();
        trade_dict.set_item("BT", self.0.BT).unwrap();

        trade_dict.into()
    }
}


pub struct TickerWrapper<'a>(pub &'a FutureTicker<'a>);

impl<'a> ToPyObject for TickerWrapper<'a> {
    fn to_object(&self, py: Python) -> PyObject {
        let ticker_dict = PyDict::new(py);
        ticker_dict.set_item("symbol", self.0.symbol).unwrap();
        ticker_dict.set_item("tickDirection", self.0.tick_direction).unwrap();
        ticker_dict.set_item("price24hPcnt", self.0.price_24h_pcnt).unwrap();
        ticker_dict.set_item("lastPrice", self.0.last_price).unwrap();
        ticker_dict.set_item("prevPrice24h", self.0.prev_price_24h).unwrap();
        ticker_dict.set_item("highPrice24h", self.0.high_price_24h).unwrap();
        ticker_dict.set_item("lowPrice24h", self.0.low_price_24h).unwrap();
        ticker_dict.set_item("prevPrice1h", self.0.prev_price_1h).unwrap();
        ticker_dict.set_item("markPrice", self.0.mark_price).unwrap();
        ticker_dict.set_item("indexPrice", self.0.index_price).unwrap();
        ticker_dict.set_item("openInterest", self.0.open_interest).unwrap();
        ticker_dict.set_item("openInterestValue", self.0.open_interest_value).unwrap();
        ticker_dict.set_item("turnover24h", self.0.turnover_24h).unwrap();
        ticker_dict.set_item("volume24h", self.0.volume_24h).unwrap();
        ticker_dict.set_item("nextFundingTime", self.0.next_funding_time).unwrap();
        ticker_dict.set_item("fundingRate", self.0.funding_rate).unwrap();
        ticker_dict.set_item("bid1Price", self.0.bid1_price).unwrap();
        ticker_dict.set_item("bid1Size", self.0.bid1_size).unwrap();
        ticker_dict.set_item("ask1Price", self.0.ask1_price).unwrap();
        ticker_dict.set_item("ask1Size", self.0.ask1_size).unwrap();

        ticker_dict.into()
    }
}


struct OrderbookItemWrapper<'a>(&'a OrderbookItem<'a>);

impl<'a> ToPyObject for OrderbookItemWrapper<'a> {
    fn to_object(&self, py: Python) -> PyObject {
        PyTuple::new(py, &[self.0.0, self.0.1]).into()
    }
}

pub struct OrderbookWrapper<'a>(pub &'a Orderbook<'a>);

impl<'a> ToPyObject for OrderbookWrapper<'a> {
    fn to_object(&self, py: Python) -> PyObject {
        let orderbook_dict: &PyDict = PyDict::new(py);
        orderbook_dict.set_item("s", self.0.s).unwrap();
        
        let bids: Vec<_> = self.0.b.iter().map(|item: &OrderbookItem<'_>| {
            OrderbookItemWrapper(item)
        }).collect();
        let asks: Vec<_> = self.0.a.iter().map(|item: &OrderbookItem<'_>| {
            OrderbookItemWrapper(item)
        }).collect();
        
        orderbook_dict.set_item("b", bids).unwrap();
        orderbook_dict.set_item("a", asks).unwrap();

        orderbook_dict.set_item("u", self.0.u).unwrap();
        if let Some(seq) = self.0.seq {
            orderbook_dict.set_item("seq", seq).unwrap();
        }

        orderbook_dict.into()
    }
}


pub struct SpotTickerWrapper<'a>(pub &'a SpotTicker<'a>);

impl<'a> ToPyObject for SpotTickerWrapper<'a> {
    fn to_object(&self, py: Python) -> PyObject {
        let ticker_dict = PyDict::new(py);
        ticker_dict.set_item("symbol", self.0.symbol).unwrap();
        ticker_dict.set_item("lastPrice", self.0.last_price).unwrap();
        ticker_dict.set_item("highPrice24h", self.0.high_price_24h).unwrap();
        ticker_dict.set_item("lowPrice24h", self.0.low_price_24h).unwrap();
        ticker_dict.set_item("prevPrice24h", self.0.prev_price_24h).unwrap();
        ticker_dict.set_item("volume24h", self.0.volume_24h).unwrap();
        ticker_dict.set_item("turnover24h", self.0.turnover_24h).unwrap();
        ticker_dict.set_item("price24hPcnt", self.0.price_24h_pcnt).unwrap();
        ticker_dict.set_item("usdIndexPrice", self.0.usd_index_price).unwrap();

        ticker_dict.into()
    }
}