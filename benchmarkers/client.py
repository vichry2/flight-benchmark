import time
import pyarrow.flight as flight
import pandas as pd

PY_PORT = 8815
RS_PORT = 50051

ITERATIONS = 10

def run_iterations(port: int):
    ticket = flight.Ticket(b"example_ticket")
    
    client = flight.FlightClient(f"grpc://localhost:{port}")
    
    data = client.do_get(ticket).read_all()
    
    nbytes_mb = data.nbytes / (1024*1024)
    
    times = []
    for _ in range(ITERATIONS):
        start_time = time.perf_counter()
        data = client.do_get(ticket).read_all()
        times.append(time.perf_counter() - start_time)
    
    avg = sum(times)/len(times)
    return avg, nbytes_mb

def main():
    avg_py, nbytes_py = run_iterations(PY_PORT)
    avg_rs, nbytes_rs = run_iterations(RS_PORT)
    
    print(f"FOR {ITERATIONS} iterations\n")
    print("Python Flight Server:")
    print(f"    size: {nbytes_py} MB, avg_time: {avg_py}")
    print("Rust Flight Server:")
    print(f"    size: {nbytes_rs} MB, avg_time: {avg_rs}")
    
if __name__ == "__main__":
    main()
