use once_cell::sync::Lazy;

use chocoapi::configuration;
use chocoapi::startup::Application;
use chocoapi::telemetry::{get_subscriber, init_subscriber};

use crate::wrappers::{TestAPI, TestConfiguration};

use super::services::TestDatabase;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = tracing::Level::INFO;
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber).unwrap();
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber).unwrap();
    };
});

/// Each test has its own instance of TestApp.
pub struct TestApp {
    /// The API address.
    pub address: String,
    /// The API port.
    pub port: u16,
    /// The database to use in tests.
    pub db: TestDatabase,
    /// An http client to be used to hit the API during tests.
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        Lazy::force(&TRACING);

        // Randomise configuration to ensure test isolation
        let configuration = {
            let environment = configuration::get_environment().expect("failed to get environment");
            let c = configuration::extract(environment).expect("Failed to read configuration.");
            TestConfiguration::new(c)
        };

        // Create the test database
        let db = TestDatabase::new(&configuration).await;

        // Launch the application as a background task
        let (address, port) = {
            let application = TestAPI::new(configuration).await;

            let local_address = application.local_address();

            let application: Application = application.into();
            let _ = tokio::spawn(application.run_until_stopped());

            (
                format!("http://{}:{}", local_address.ip(), local_address.port()),
                local_address.port(),
            )
        };

        let api_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .unwrap();

        TestApp {
            address,
            port,
            db,
            api_client,
        }
    }
}
