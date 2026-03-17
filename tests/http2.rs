#![cfg(feature = "clob")]

use polymarket_client_sdk::clob::Config;

#[test]
fn clob_config_force_http2_defaults_off_and_can_be_enabled() {
    let default_cfg = Config::default();
    assert!(format!("{default_cfg:?}").contains("force_http2: false"));

    let force_http2_cfg = Config::builder().force_http2(true).build();
    assert!(format!("{force_http2_cfg:?}").contains("force_http2: true"));
}
