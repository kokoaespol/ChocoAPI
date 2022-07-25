use std::convert::From;
use std::ops::Deref;

use sqlx::postgres::PgConnectOptions;
use uuid::Uuid;

use chocoapi::configuration::Settings;
use chocoapi::startup::Application;

pub struct TestAPI(Application);

impl TestAPI {
    pub async fn new(configuration: TestConfiguration) -> Self {
        let application = Application::build(configuration.into())
            .await
            .expect("failed to build application");
        TestAPI(application)
    }
}

impl Deref for TestAPI {
    type Target = Application;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TestAPI> for Application {
    fn from(w: TestAPI) -> Application {
        w.0
    }
}

#[derive(Clone)]
pub struct TestConfiguration(Settings);

impl TestConfiguration {
    pub fn new(mut config: Settings) -> Self {
        config.application.port = 0;
        config.database.host = "localhost".to_string();
        config.database.database_name = Uuid::new_v4().to_string();
        TestConfiguration(config)
    }
}

impl TestConfiguration {
    pub fn db_name(&self) -> &str {
        &self.database.database_name
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.database.with_db()
    }

    pub fn without_db(&self) -> PgConnectOptions {
        self.database.without_db()
    }
}

impl Deref for TestConfiguration {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TestConfiguration> for Settings {
    fn from(w: TestConfiguration) -> Settings {
        w.0
    }
}
