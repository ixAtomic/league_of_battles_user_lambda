use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lib::models::user_model::UserResponseModel;
use lib::service::users_service;
use serde::de::IntoDeserializer;
use serde::Deserialize;

#[derive(Deserialize)]
struct UserRequest {
    id: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // Return something that implements IntoResponse.
    let user_request: UserRequest = serde_json::from_slice(_event.body())?;

    let _res = users_service::get_user_data(&user_request.id).await?;

    let body = format!("request body {:?}", _res);

    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&_res)?.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
