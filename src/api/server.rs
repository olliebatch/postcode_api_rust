use crate::Config;

pub async fn start_api(config: Config) -> tide::Result<()> {
    let port = config.port;

    let mut app = tide::new();

    app.listen(format!("127.0.0.1:{}", port)).await?;

    Ok(())
}
