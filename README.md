### Setup

1. **Install dependencies**:
    - pip install any packages needed for `flight_py`
    - `cargo build --release` for `flight_rs`

2. **Environment variables**: I've set up some environment variables to configure the amount of data the servers send to the client:
    - `NUM_ROWS` --> number of rows to hold in memory and to send to client (applicable for Python and Rust server)
    - `NUM_COLUMNS` --> number of columns to hold in memory and to send to client (applicable for Python and Rust server)
    - `MAX_ROWS` --> maximum number of rows in a single record batch (chunking) (applicable for Rust server)
> **Note**: I've observed that the higher the data size sent to the client (high `NUM_ROWS` and `NUM_COLUMNS` values), the higher the difference between Python and Rust server response times.

3. Run `flight_py/flight.py` script in one session, and Rust release binary in another.

4. Run `benchmarkers/client.py` to view comparative results.
