#![allow(unused)]

use anyhow::Result;
use serde_json::json;
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    // hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Vishal").await?.print().await?;
    // hc.do_get("/hello2/Vishal").await?.print().await?;

    // hc.do_get("/Cargo.toml").await?.print().await?;

    // login
    let req_login = hc
        .do_post(
            "/api/login",
            json!({
            "username": "demo",
            "pwd": "test"
            }),
        )
        .await
        .unwrap()
        .print()
        .await;

    // let req_login = hc
    //     .do_post(
    //         "/api/login",
    //         json!({
    //         "username": "demo",
    //         "pwd": "test"
    //         }),
    //     )
    //     .await
    //     .unwrap()
    //     .print()
    //     .await;

    Ok(())
}
