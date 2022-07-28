use reqwest::multipart;

use crate::helpers::TestApp;

use chocoapi::models::User;

#[tokio::test]
async fn hitting_register_with_valid_data_returns_created_and_new_user_as_json() {
    // Arrange
    let app = TestApp::new().await;
    let client = reqwest::Client::new();
    let form_data = multipart::Form::new()
        .text("username", "johndoe")
        .text("password", "12345")
        .text("full_name", "John Doe")
        .text("email", "john@doe.com");

    // Act
    let response = client
        .post(&format!("{}/register", &app.address))
        .multipart(form_data)
        .send()
        .await
        .expect("failed to execute request");

    let response_status = response.status();
    let created_user: User = response
        .json()
        .await
        .expect("failed to parse user from server response");

    // Assert
    assert!(response_status.is_success());
    assert!(created_user.active);
    assert_eq!(Some("John Doe".to_string()), created_user.full_name);
    assert_eq!("johndoe".to_string(), created_user.username);

    // TODO: check that the user is persisted to the database and
    // that their email is not confirmed
}
