use serial_test::parallel;
use crate::tests::{get_client, get_self_user};

#[tokio::test]
#[parallel]
async fn test_self_user() {
    let client = get_client();
    let _ = get_self_user(&client).await;
}

#[tokio::test]
#[parallel]
async fn test_user() {
    let client = get_client();
    let uid = get_self_user(&client).await.uid;
    let user = client.get_user(&uid).await;
    let user = user.expect("user");
    assert!(user.exists);
    assert_eq!(user.id, uid);
    let user_data = user.content.expect("user data");
    assert_eq!(user_data.uid, user.id);
}

#[tokio::test]
#[parallel]
async fn test_update_user() {
    let client = get_client();
    let mut user = get_self_user(&client).await;
    user.color = Some("#ff0000".to_string());
    assert!(client.update_user(&user).await.is_ok());

    let user = get_self_user(&client).await;
    assert_eq!(user.color, Some("#ff0000".to_string()));

    let mut user = get_self_user(&client).await;
    user.color = Some("#00ff00".to_string());
    assert!(client.update_user(&user).await.is_ok());

    let user = get_self_user(&client).await;
    assert_eq!(user.color, Some("#00ff00".to_string()));
}

#[tokio::test]
#[parallel]
async fn test_set_username() {
    let client = get_client();
    let user = get_self_user(&client).await;

    let new_username = format!("{}test", user.username);
    let resp = client.set_username(&user.uid, &new_username).await;
    let resp = resp.expect("set username");
    assert!(resp.success);
    assert!(resp.message.is_none());

    let new_user = get_self_user(&client).await;
    assert_eq!(new_user.username, new_username);

    let resp = client.set_username(&user.uid, &user.username).await;
    let resp = resp.expect("reset username");
    assert!(resp.success);
    assert!(resp.message.is_none());

    let new_user = get_self_user(&client).await;
    assert_eq!(new_user.username, user.username);
}

#[tokio::test]
#[parallel]
async fn test_set_invalid_username() {
    let client = get_client();
    let uid = get_self_user(&client).await.uid;

    let resp = client.set_username(uid, "0").await;
    let resp = resp.expect("set invalid username");
    assert!(!resp.success);
    assert!(resp.message.is_some());
}