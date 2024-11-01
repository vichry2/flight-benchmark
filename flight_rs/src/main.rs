use arrow_flight::encode::FlightDataEncoderBuilder;
use futures::{StreamExt, TryStreamExt};
use futures::stream::{BoxStream, self};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};
use std::str::FromStr;

use arrow_flight::{
    flight_service_server::FlightService, flight_service_server::FlightServiceServer, Action,
    ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo, HandshakeRequest,
    HandshakeResponse, PollInfo, PutResult, SchemaResult, Ticket,
};
use arrow::array::{ArrayRef, Int32Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct FlightServiceImpl {
    batch: RecordBatch
}

#[tonic::async_trait]
impl FlightService for FlightServiceImpl {
    type HandshakeStream = BoxStream<'static, Result<HandshakeResponse, Status>>;
    type ListFlightsStream = BoxStream<'static, Result<FlightInfo, Status>>;
    type DoGetStream = BoxStream<'static, Result<FlightData, Status>>;
    type DoPutStream = BoxStream<'static, Result<PutResult, Status>>;
    type DoActionStream = BoxStream<'static, Result<arrow_flight::Result, Status>>;
    type ListActionsStream = BoxStream<'static, Result<ActionType, Status>>;
    type DoExchangeStream = BoxStream<'static, Result<FlightData, Status>>;

    async fn handshake(
        &self,
        _request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        Err(Status::unimplemented("Implement handshake"))
    }

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        Err(Status::unimplemented("Implement list_flights"))
    }

    async fn get_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        Err(Status::unimplemented("Implement get_flight_info"))
    }

    async fn poll_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<PollInfo>, Status> {
        Err(Status::unimplemented("Implement poll_flight_info"))
    }

    async fn get_schema(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        Err(Status::unimplemented("Implement get_schema"))
    }

    async fn do_get(
        &self,
        _request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {

        let size = self.batch.num_rows();

        let stream = stream::iter(vec![self.batch.slice(0, size)]).map(Ok);

        let fd = FlightDataEncoderBuilder::new().build(stream).map_err(|e| Status::internal(e.to_string()));

        Ok(Response::new(Box::pin(fd)))
    }

    async fn do_put(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        Err(Status::unimplemented("Implement do_put"))
    }

    async fn do_action(
        &self,
        _request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        Err(Status::unimplemented("Implement do_action"))
    }

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        Err(Status::unimplemented("Implement list_actions"))
    }

    async fn do_exchange(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("Implement do_exchange"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let n_columns = env::var("NUM_COLUMNS").ok()
        .and_then(|v| usize::from_str(&v).ok())
        .unwrap_or(20);
    
    let n_rows = env::var("NUM_ROWS").ok()
        .and_then(|v| usize::from_str(&v).ok())
        .unwrap_or(500_000);

    let service = FlightServiceImpl {
        batch: generate_record_batch(n_columns, n_rows).unwrap()
    };

    let svc = FlightServiceServer::new(service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

fn generate_record_batch(n: usize, m: usize) -> Result<RecordBatch, arrow::error::ArrowError> {
    // Define the schema: n columns, each with Int32 data type
    let fields: Vec<Field> = (0..n)
        .map(|i| Field::new(&format!("col{}", i), DataType::Int32, false))
        .collect();
    let schema = Arc::new(Schema::new(fields));

    // Generate m rows of dummy data for each column
    let columns: Vec<ArrayRef> = (0..n)
        .map(|_| {
            let array: Int32Array = (0..m).map(|i| i as i32).collect();
            Arc::new(array) as ArrayRef
        })
        .collect();

    // Create the record batch with the generated schema and columns
    RecordBatch::try_new(schema, columns)
}