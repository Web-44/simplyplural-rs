use serial_test::parallel;
use crate::handler::front::model::FrontData;
use crate::tests::{get_client, test_add_document, test_document_action, test_empty_documents, test_get_document, test_get_documents};
use crate::UnixTimestamp;

#[tokio::test]
#[parallel]
async fn test_empty_front() {
    let client = get_client();

    test_empty_documents(client.get_current_fronters().await);
}

#[cfg(all(feature = "members", feature = "users"))]
#[tokio::test]
#[parallel]
async fn test_front_history() {
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::tests::{get_self_user, get_test_member, cleanup_test_member};

    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_millis();
    let time = time as UnixTimestamp;
    let front_start_time = time - 10000;
    let interval_start_time = time - 20000;
    let interval_end_time = time + 10000;

    let client = get_client();
    let uid = get_self_user(&client).await.uid;
    let member_id = get_test_member(&client).await;

    test_empty_documents(client.get_current_fronters().await);

    let entry_id = test_add_document(client.add_member_to_front(&member_id, front_start_time, Some("Testing".to_string())).await);
    test_get_documents(client.get_current_fronters().await, &entry_id, |entry| check_front_entry(entry, &member_id, front_start_time, true));

    test_document_action(client.remove_member_from_front(&entry_id, time).await);
    test_empty_documents(client.get_current_fronters().await);

    test_get_documents(client.get_front_history(&uid, interval_start_time, interval_end_time).await, &entry_id, |entry| check_front_entry(entry, &member_id, front_start_time, false));
    test_get_documents(client.get_member_front_history(&member_id).await, &entry_id, |entry| check_front_entry(entry, &member_id, front_start_time, false));
    test_get_documents(client.get_member_front_history_in_interval(&member_id, interval_start_time, interval_end_time).await, &entry_id, |entry| check_front_entry(entry, &member_id, front_start_time, false));
    test_get_document(client.get_front_history_entry(&uid, &entry_id).await, &entry_id, |entry| check_front_entry(entry, &member_id, front_start_time, false));

    test_document_action(client.delete_front_entry(&entry_id).await);
    test_empty_documents(client.get_member_front_history(&member_id).await);

    cleanup_test_member(&client, &member_id).await;
}

fn check_front_entry(front_data: &FrontData, member_id: &str, front_start_time: UnixTimestamp, live: bool) {
    assert_eq!(front_data.member, member_id);
    assert_eq!(front_data.start_time, front_start_time);
    assert_eq!(front_data.custom_status, "Testing");
    assert_eq!(front_data.live, live);
    assert!(!front_data.custom);
}