import random 
import time

import pyarrow.flight as fl
from locust import User, constant_throughput, events, task

PORT = 8815

class FlightUser(User):
    abstract = True
    port = PORT
    location = "localhost"
    wait_time = constant_throughput(1)
    host = "http://dummy-host.com"
    
    def on_start(self):
        self.client = fl.FlightClient((self.location, self.port))
        
        return super().on_start()
    
    @task
    def send_flight_request(self):
        ticket = fl.Ticket(b"example_ticket")
        
        start_time = time.perf_counter()
        
        self.client.do_get(ticket).read_all()
        
        resp_time = time.perf_counter() - start_time
        
        events.request.fire(
            request_type = "Locust Test",
            name = "Locust Test",
            response_time = resp_time,
            response_length = 0,
            exception = None
        )
        
class FlightGrpcUser(FlightUser):
    pass