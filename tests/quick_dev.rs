

use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/hello?name=Jen").await?.print().await?;
    // hc.do_get("/hello2/mike").await?.print().await?;

    // hc.do_post(
    //     "/api/login",
    //     json!({
    //         "username": "demo1",
    //         "pwd": "welcome"
    //     }),
    // ).await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "New ticket"
        }),
    ).await?.print().await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // static get
    // hc.do_get("/src/main.rs").await?.print().await?;



    Ok(())
}