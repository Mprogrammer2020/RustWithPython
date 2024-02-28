import crypto_rust
import asyncio

def ticker_handler(dict_from_rust):
    print('python got this dict from rust for ticker handler ' + str(dict_from_rust))

def trade_handler(dict_from_rust):
    print('python got this dict from rust for trade handler ' + str(dict_from_rust))

def orderbook_handler(dict_from_rust):
    print('python got this dict from rust for orderbook handler ' + str(dict_from_rust))

def usdm_handler(dict_from_rust):
    print('python got this dict from rust for usdm handler ' + str(dict_from_rust))

def coinm_handler(dict_from_rust):
    print('python got this dict from rust for coinm handler ' + str(dict_from_rust))


async def t1_event_watcher():
    # Params: symbol, ticker_sensitivity, ticker_handler(function)
    await crypto_rust.binance_ticker_watcher("bnbusdt", 1.0, ticker_handler)

async def t2_event_watcher():
    # Params: symbol, trade_sensitivity, trade_handler(function)
    await crypto_rust.binance_trade_watcher("btcusdt", 1.0, trade_handler)

async def t3_event_watcher():
    # Params: symbol, mtype: coinm/usdm, usdm_handler(function)
    await crypto_rust.binance_future_m_ticker("btcusdt", "usdm", usdm_handler)

async def t4_event_watcher():
    # Params: symbol, mtype: coinm/usdm, coinm_handler(function)
    await crypto_rust.binance_future_m_ticker("btcusd_perp", "coinm", coinm_handler)

async def t5_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_trade_watcher("ETHUSD", "inverse", trade_handler)

async def t6_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_trade_watcher("ETHUSDT", "linear", trade_handler)

async def t7_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_ticker_watcher("ETHUSD", "inverse", ticker_handler)

async def t8_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_ticker_watcher("ETHUSDT", "linear", ticker_handler)

async def t9_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_orderbook_watcher("ETHUSD", "inverse", orderbook_handler)

async def t10_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_orderbook_watcher("ETHUSDT", "linear", orderbook_handler)

async def t11_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_spot_trade_watcher("ETHUSDT", trade_handler)

async def t12_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_spot_ticker_watcher("ETHUSDT", ticker_handler)

async def t13_event_watcher():
    # Params: symbol, channel_type: linear/inverse, handler(function)
    await crypto_rust.bybit_spot_orderbook_watcher("ETHUSDT", orderbook_handler)

# asyncio.run(t1_event_watcher())
# asyncio.run(t2_event_watcher())
# asyncio.run(t3_event_watcher())
# asyncio.run(t4_event_watcher())
# asyncio.run(t5_event_watcher())
# asyncio.run(t6_event_watcher())
# asyncio.run(t7_event_watcher())
# asyncio.run(t8_event_watcher())
# asyncio.run(t9_event_watcher())
# asyncio.run(t10_event_watcher())
# asyncio.run(t11_event_watcher())
asyncio.run(t12_event_watcher())
# asyncio.run(t13_event_watcher())
