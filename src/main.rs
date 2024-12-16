use anyhow::Result;

mod block;
mod config;
mod file;
mod filter;
mod server;
mod tokenizer;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match config::new_from_file(config::DEFAULT_CONFIG_PATH) {
        Ok(config) => config,
        Err(e) => {
            if e.downcast_ref::<std::io::Error>()
                .map_or(false, |e| e.kind() == std::io::ErrorKind::NotFound)
            {
                config::Config::default()
            } else {
                return Err(e);
            }
        }
    };

    server::server::start_server(config).await;
    Ok(())
}
