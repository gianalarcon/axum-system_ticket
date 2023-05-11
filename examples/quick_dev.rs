use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8081")?;
  hc.do_get("/hello2/Mike").await?.print().await?;
  hc.do_get("/src/main.rs").await?.print().await?;
  Ok(())
}
