use bybit::ws::future;
use pyo3::prelude::*;
use bybit::ws::response::{FuturePublicResponse, Trade};
use bybit::WebSocketApiClient;
use pyo3::types::*;
use crate::bybit_response::{TradeWrapper, OrderbookWrapper, TickerWrapper};


pub async fn trade_stream(symbol: String, channel_type: String, handler: PyObject) {
    
    let mut client = if channel_type=="inverse" {WebSocketApiClient::future_inverse().build()} else {WebSocketApiClient::future_linear().build()};

    client.subscribe_trade(symbol);

    if let Err(e) = client.run(|event: FuturePublicResponse<'_>| match event {
        FuturePublicResponse::Trade(trade) => {
            Python::with_gil(|py: Python<'_>| {
                let ticker_dict: &PyDict = PyDict::new(py);
                ticker_dict.set_item("topic", trade.topic).unwrap();
                ticker_dict.set_item("type", trade.type_).unwrap();
                ticker_dict.set_item("ts", trade.ts).unwrap();
                let trades_list: Vec<TradeWrapper> = trade.data.iter().map(|t: &Trade<'_>| TradeWrapper(t)).collect();
                let trades_list: &PyList = PyList::new(py, &trades_list);
                ticker_dict.set_item("data", trades_list).unwrap();
                let _ = handler.call1(py, (ticker_dict,)).unwrap();
            });
        }
        _ => (),
    }) {
        println!("ByBit Trade Error: {e}");
    };
}


pub async fn ticker_stream(symbol: String, channel_type: String, handler: PyObject) {

    let mut client: future::FutureWebsocketApiClient = if channel_type=="inverse" {WebSocketApiClient::future_inverse().build()} else {WebSocketApiClient::future_linear().build()};

    client.subscribe_ticker(symbol);

    if let Err(e) = client.run(|event: FuturePublicResponse<'_>| match event {
        FuturePublicResponse::Ticker(ticker) => {
            Python::with_gil(|py: Python<'_>| {
                let ticker_dict: &PyDict = PyDict::new(py);
                ticker_dict.set_item("topic", ticker.topic).unwrap();
                ticker_dict.set_item("type", ticker.type_).unwrap();
                ticker_dict.set_item("cs", ticker.cs).unwrap();
                ticker_dict.set_item("ts", ticker.ts).unwrap();
                let ticker_data_dict: TickerWrapper<'_> = TickerWrapper(&ticker.data);
                ticker_dict.set_item("data", ticker_data_dict).unwrap();
                let _ = handler.call1(py, (ticker_dict,)).unwrap();
            });
        },
        _ => (),
    }) {
        println!("ByBit Ticker Error: {e}");
    }
}


pub async fn orderbook_stream(symbol: String, channel_type: String, handler: PyObject) {
    let mut client = if channel_type=="inverse" {WebSocketApiClient::future_inverse().build()} else {WebSocketApiClient::future_linear().build()};

    client.subscribe_orderbook(symbol, future::OrderbookDepth::Level1);

    if let Err(e) = client.run(|event: FuturePublicResponse<'_>| match event {
        FuturePublicResponse::Orderbook(orderbook) => {
            Python::with_gil(|py: Python<'_>| {
                let order_dict: &PyDict = PyDict::new(py);
                order_dict.set_item("topic", orderbook.topic).unwrap();
                order_dict.set_item("type", orderbook.type_).unwrap();
                order_dict.set_item("ts", orderbook.ts).unwrap();
                let order_data_dict: OrderbookWrapper<'_> = OrderbookWrapper(&orderbook.data);
                order_dict.set_item("data", order_data_dict).unwrap();
                let _ = handler.call1(py, (order_dict,)).unwrap();
            });
        },
        _ => (),
    }) {
        println!("ByBit Orderbook Error: {e}");
    }
}