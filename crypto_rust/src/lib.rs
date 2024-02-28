use pyo3::prelude::*;
pub mod  binance_ws;
pub mod binance_future_ws;
pub mod bybit_ws;
pub mod bybit_spot_ws;
pub mod bybit_response;

#[pyfunction]
fn binance_trade_watcher(py: Python, symbol: String, trade_sensitivity: f64, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        binance_ws::trade_stream(symbol.to_lowercase(), trade_handler, trade_sensitivity).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn binance_ticker_watcher(py: Python, symbol: String, ticker_sensitivity: f64, ticker_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        binance_ws::ticker_stream(symbol.to_lowercase(), ticker_handler, ticker_sensitivity).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn binance_future_m_ticker(py: Python, symbol: String, mtype: String, ticker_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        binance_future_ws::future_ticker_m_stream(symbol.to_lowercase(), mtype, ticker_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn bybit_trade_watcher(py: Python, symbol: String, channel_type: String, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit_ws::trade_stream(symbol.to_uppercase(), channel_type, trade_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn bybit_ticker_watcher(py: Python, symbol: String, channel_type: String, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit_ws::ticker_stream(symbol.to_uppercase(), channel_type, trade_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn bybit_orderbook_watcher(py: Python, symbol: String, channel_type: String, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit_ws::orderbook_stream(symbol.clone().to_uppercase(), channel_type, trade_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn bybit_spot_trade_watcher(py: Python, symbol: String, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit_spot_ws::trade_stream(symbol.clone().to_uppercase(), trade_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn bybit_spot_ticker_watcher(py: Python, symbol: String, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit_spot_ws::ticker_stream(symbol.clone().to_uppercase(), trade_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pyfunction]
fn bybit_spot_orderbook_watcher(py: Python, symbol: String, trade_handler: PyObject) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        bybit_spot_ws::orderbook_stream(symbol.clone().to_uppercase(), trade_handler).await;
        Ok(Python::with_gil(|py: Python<'_>| py.None()))
    })
}

#[pymodule]
fn crypto_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(binance_trade_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(binance_ticker_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(binance_future_m_ticker, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_trade_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_ticker_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_orderbook_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_spot_trade_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_spot_ticker_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(bybit_spot_orderbook_watcher, m)?)?;
    Ok(())
}
