import pyarrow as pa
import pyarrow.flight as fl
import os

class SimpleFlightService(fl.FlightServerBase):
    def __init__(self, table, location="grpc://localhost:8815"):
        self.table = table
        super().__init__(location)

    def do_get(self, context, ticket):
        return fl.RecordBatchStream(self.table)
    
def generate_table(n: int, m: int) -> pa.Table:
    fields = [("col{}".format(i), pa.int32()) for i in range(n)]
    
    columns = []
    for _ in range(n):
        array = pa.array([i for i in range(m)], type=pa.int32())
        columns.append(array)
    
    return pa.table(columns, names=[name for name, _ in fields])
    
if __name__ == "__main__":
    n_columns = int(os.environ.get("NUM_COLUMNS", 20))
    n_rows = int(os.environ.get("NUM_ROWS", 500000))
    
    server = SimpleFlightService(table=generate_table(n_columns, n_rows))
    print("Starting Flight server on localhost:8815")
    server.serve()
