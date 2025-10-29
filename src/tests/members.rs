use crate::handler::members::model::{MemberCreateData, MemberData};
use crate::tests::{cleanup_test_member, get_client, get_self_user, get_test_member, test_document_action, test_empty_documents, test_get_document, test_get_documents};
use serial_test::serial;

#[cfg(feature = "users")]
#[tokio::test]
#[serial]
async fn test_members() {
    let client = get_client();
    let uid = get_self_user(&client).await.uid;
    let member_id = get_test_member(&client).await;

    test_get_documents(client.get_all_members(&uid).await, &member_id, check_member);
    test_get_document(client.get_member(&uid, &member_id).await, &member_id, check_member);

    test_document_action(client.update_member(&member_id, &MemberCreateData {
        name: "updated".to_string(),
        description: Some("updated member".to_string()),
        description_markdown: None,
        pronouns: Some("she/her".to_string()),
        pluralkit_id: None,
        avatar: None,
        private: None,
        prevent_trusted: None,
        prevents_front_notifications: None,
        color: Some("#ff00ff".to_string()),
        info: None,
    }).await);

    test_get_document(client.get_member(&uid, &member_id).await, &member_id, |member| {
        assert_eq!(member.name, "updated");
        assert_eq!(member.description, "updated member");
        assert_eq!(member.pronouns, "she/her");
        assert_eq!(member.color, "#ff00ff");
    });

    cleanup_test_member(&client, &member_id).await;
    test_empty_documents(client.get_all_members(&uid).await);
}

fn check_member(member: &MemberData) {
    assert_eq!(member.name, "test");
    assert_eq!(member.description, "test member");
    assert_eq!(member.pronouns, "they/them");
    assert_eq!(member.color, "#00ffff");
}