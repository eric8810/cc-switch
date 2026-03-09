use std::net::TcpListener;

use cc_switch_lib::scan_local_proxies;

use super::support::{reset_baseline_fs, test_mutex};

fn bind_proxy_test_port() -> (u16, TcpListener) {
    for port in [7890_u16, 7891, 8080, 8888, 3128, 10808, 10809, 1080] {
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
            return (port, listener);
        }
    }
    panic!("no free proxy test port available");
}

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn global_proxy_baseline_scan_local_ports_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let (port, _listener) = bind_proxy_test_port();

    let proxies = scan_local_proxies().await;
    assert!(
        proxies.iter().any(|proxy| proxy.port == port),
        "scan should discover the temporary listener on port {port}"
    );
}
