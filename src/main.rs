use chocoapi::{
    configuration::{self, Environment},
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use eyre::{Result, WrapErr};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    let environment = configuration::get_environment()?;

    // change log format on local environment to improve human readibility
    match environment {
        Environment::Local => {
            let env_filter = EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env_lossy();
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .finish();
            init_subscriber(subscriber)?;
        }
        Environment::Production => {
            let subscriber = get_subscriber(
                "chocoapi".to_string(),
                tracing::Level::INFO,
                std::io::stdout,
            );
            init_subscriber(subscriber)?;
        }
    };

    let configuration =
        configuration::extract(environment).wrap_err("failed to read configuration")?;

    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;

    Ok(())
}
