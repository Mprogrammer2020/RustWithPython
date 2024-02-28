use pyo3::prelude::*;
use binance::futures::websockets::*;
use std::sync::atomic::AtomicBool;
use pyo3::types::*;


pub async fn future_ticker_m_stream(symbol: String, mtype: String, handler: PyObject) {
    let keep_running: AtomicBool = AtomicBool::new(true);
    let agg_trade: String = format!("{}@bookTicker", symbol);
    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
        match event {
            FuturesWebsocketEvent::BookTicker(ticker) => {
                Python::with_gil(|py: Python<'_>| {
                    let ticker_dict: &PyDict = PyDict::new(py);
                    ticker_dict.set_item("u", ticker.update_id).unwrap();
                    ticker_dict.set_item("s", ticker.symbol).unwrap();
                    ticker_dict.set_item("b", ticker.best_bid).unwrap();
                    ticker_dict.set_item("B", ticker.best_bid_qty).unwrap();
                    ticker_dict.set_item("a", ticker.best_ask).unwrap();
                    ticker_dict.set_item("A", ticker.best_ask_qty).unwrap();
                    let _ = handler.call1(py, (ticker_dict,)).unwrap();
                });
            },
            _ => (),
        };
        Ok(())
    });

    web_socket.connect(if mtype=="coinm" {&FuturesMarket::COINM} else {&FuturesMarket::USDM}, &agg_trade).unwrap();
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Future Ticker Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("Future Ticker Disconnected");
}