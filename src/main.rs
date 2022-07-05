use chocoapi::{
    configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use eyre::{Result, WrapErr};

#[tokio::main]
async fn main() -> Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    let subscriber = get_subscriber("chocoapi".into(), tracing::Level::INFO, std::io::stdout);
    init_subscriber(subscriber)?;

    let configuration = configuration::extract().wrap_err("failed to read configuration")?;

    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;

    Ok(())
}
