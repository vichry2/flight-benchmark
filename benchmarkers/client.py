from concurrent.futures import ThreadPoolExecutor, as_completed
import time
import pyarrow.flight as flight
import pandas as pd

PORT = 8815

def fetch_data():
    ticket = flight.Ticket(b"example_ticket")
    
    client = flight.FlightClient(f"grpc://localhost:{PORT}")
    
    start_time = time.perf_counter()
    data = client.do_get(ticket).read_all()
    print(f"TOOK: {time.perf_counter() - start_time}")
    
    print(f"NUM BYTES: {data.nbytes /(1024*1024)} MB")
    print(data.to_pandas())

    

if __name__ == "__main__":
    fetch_data()
