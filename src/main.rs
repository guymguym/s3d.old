mod api;
mod layers;

use crate::api::*;

#[tokio::main]
pub async fn main() -> Result<(), SyncError> {
    Ok(serve().await?)
}
