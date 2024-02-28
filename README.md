# Rust With Python

## Description:
This is Python Crypto project powered by Rust code! This project combines the robustness of Rust with the versatility of Python to create a powerful tool for crypto enthusiasts. The project includes integration with some websocket functions of popular exchanges like Binance and Bybit, making it a handy tool for real-time data analysis and trading.

With the use of Maturin library, we have integrated the Rust code into Python project. This allows us to use the fast performance and safety features of Rust while maintaining the flexibility and ease of use of Python.

## Steps to Setup the Project:

1. **Create a Python Virtual Environment:** 
   ```bash
   python3.10 -m venv env
   ```

2. **Install Requirements:**
   ```bash
   pip install -r requirements.txt
   ```

3. **Build and Install the Rust Module:**
   Navigate to the `crypto_rust` directory and run:
   ```bash
   maturin develop --release
   ```

4. **Run the Project:**
   With all dependencies installed and the Rust module integrated, you're ready to dive into the magic! Run the `watcher.py` file and witness the fusion of Rust and Python in action.

Happy coding! 🚀🐍🦀
