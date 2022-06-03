use embe_interface_storage::{StorageSender, Storage, HandleRequest, RequestReply};
use wasmbus_rpc::provider::prelude::*;

use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    testing::{TestOptions, TestResult},
};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, 
        health_check, 
        handle_request_test,
    );
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut dowwn gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// tests of the handle_request
async fn handle_request_test(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // create client and ctx
    let client = StorageSender::via(prov);
    let ctx = Context::default();

    let req = HandleRequest {
        capacity: 0,
        fixed_size: false,
        fill_from: None,
        max_capacity: None,
    };
    let repl = client.create_handle(&ctx, &req).await?;

    assert_eq!(repl.error_message, None);
    assert_ne!(repl.handle, None);

    Ok(())
}

