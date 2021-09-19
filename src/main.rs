mod api;
mod layers;
mod server;
mod util;

use crate::server::{s3_handler, s3_init};
use crate::util::*;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

#[tokio::main]
pub async fn main() -> Result<(), SyncError> {
    let addr = ([127, 0, 0, 1], 3000).into();
    s3_init().await;
    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(s3_handler))
    }));
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
