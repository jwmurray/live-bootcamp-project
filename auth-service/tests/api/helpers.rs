use auth_service::Application;
use auth_service::{AppState, HashmapUserStore};
use serde;
use uuid::Uuid;

use reqwest;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashmapUserStore::new_arc_rwlock();
        let app_state = AppState::new(user_store);
        let app = Application::build(app_state, "0.0.0.0:0")
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", &app.address);

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // create a reqwest http client instance
        let http_client = reqwest::Client::new();

        // Create new TestApp instance with the address and http_client
        TestApp {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to get root")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to get signup logout")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify_2fa", &self.address))
            .send()
            .await
            .expect("Failed to get signup logout")
    }

    pub async fn post_verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify_token", &self.address))
            .send()
            .await
            .expect("Failed to get signup logout")
    }

    // TODO: Implement helper functions for all other routes
    // (signup, login, logout, verify-2fa, and verify-token)
}

pub fn get_random_email() -> String {
    format!("{}@example.com", &Uuid::new_v4())
}
