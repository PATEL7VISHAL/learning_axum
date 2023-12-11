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

    let create_ticket = hc
        .do_post(
            "/api/tickets",
            json!({
                "title": "not working"
            }),
        )
        .await?
        .print()
        .await;

    // let create_ticket2 = hc
    //     .do_post(
    //         "/api/tickets",
    //         json!({
    //             "title": "Ticket 222 !!!"
    //         }),
    //     )
    //     .await?
    //     .print()
    //     .await;

    let list_ticket = hc.do_get("/api/tickets").await?.print().await;
    let delete_ticket = hc.do_delete("/api/tickets/0").await?.print().await;
    let delete_ticket = hc.do_delete("/api/tickets/0").await?.print().await;
    // let delete_ticket = hc.do_delete("/api/tickets/1").await?.print().await;

    Ok(())
}
