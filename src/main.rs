//! storage capability provider
//!
//!
use log::debug;
use wasmbus_rpc::provider::prelude::*;
use embe_interface_storage::*;

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(StorageProvider::default())?;

    eprintln!("storage provider exiting");
    Ok(())
}

/// storage capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Storage)]
struct StorageProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for StorageProvider {}
impl ProviderHandler for StorageProvider {}

/// Handle Factorial methods
#[async_trait]
impl Storage for StorageProvider {
    /// accepts a number and calculates its factorial
    async fn create_handle(&self, _ctx: &Context, _req: &HandleRequest) -> RpcResult<RequestReply> {

        let repl = RequestReply {
            handle: Some(1234),
            error_message: None
        };

        Ok(repl)
    }
}
