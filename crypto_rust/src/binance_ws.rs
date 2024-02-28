use pyo3::prelude::*;
use binance::websockets::*;
use std::sync::atomic::AtomicBool;
use pyo3::types::*;


pub async fn ticker_stream(symbol: String, handler: PyObject, _sensitivity: f64) {
    let keep_running: AtomicBool = AtomicBool::new(true);
    let agg_trade: String = format!("{}@bookTicker", symbol);

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::BookTicker(ticker_event) = event {
            Python::with_gil(|py: Python<'_>| {
                let ticker_dict: &PyDict = PyDict::new(py);
                ticker_dict.set_item("u", ticker_event.update_id).unwrap();
                ticker_dict.set_item("s", ticker_event.symbol).unwrap();
                ticker_dict.set_item("b", ticker_event.best_bid).unwrap();
                ticker_dict.set_item("B", ticker_event.best_bid_qty).unwrap();
                ticker_dict.set_item("a", ticker_event.best_ask).unwrap();
                ticker_dict.set_item("A", ticker_event.best_ask_qty).unwrap();
                let _ = handler.call1(py, (ticker_dict,)).unwrap();
            });
        };
        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap();
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Ticker Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("Ticker Disconnected");    
}


pub async fn trade_stream(symbol: String, handler: PyObject, _sensitivity: f64) {
    let keep_running: AtomicBool = AtomicBool::new(true);
    let agg_trade: String = format!("{}@trade", symbol);
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Trade(trade) => {
                Python::with_gil(|py: Python<'_>| {
                    let trade_dict: &PyDict = PyDict::new(py);
                    trade_dict.set_item("e", trade.event_type).unwrap();
                    trade_dict.set_item("E", trade.event_time).unwrap();
                    trade_dict.set_item("s", trade.symbol).unwrap();
                    trade_dict.set_item("t", trade.trade_id).unwrap();
                    trade_dict.set_item("p", trade.price).unwrap();
                    trade_dict.set_item("q", trade.qty).unwrap();
                    trade_dict.set_item("b", trade.buyer_order_id).unwrap();
                    trade_dict.set_item("a", trade.seller_order_id).unwrap();
                    trade_dict.set_item("T", trade.trade_order_time).unwrap();
                    trade_dict.set_item("m", trade.is_buyer_maker).unwrap();
                    trade_dict.set_item("M", trade.m_ignore).unwrap();
                    let _ = handler.call1(py, (trade_dict,)).unwrap();
                });
            }
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap();
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Trade Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("Trade Disconnected");
}