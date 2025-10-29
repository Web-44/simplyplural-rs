use std::time::{SystemTime, UNIX_EPOCH};
use serial_test::parallel;
use crate::handler::notes::model::{NoteCreateData, NoteData, NoteUpdateData};
use crate::tests::{cleanup_test_member, get_client, get_self_user, get_test_member, test_add_document, test_document_action, test_empty_documents, test_get_document, test_get_documents};
use crate::UnixTimestamp;

#[cfg(all(feature = "members", feature = "users"))]
#[tokio::test]
#[parallel]
async fn test_notes() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_millis();
    let time = time as UnixTimestamp;

    let client = get_client();
    let uid = get_self_user(&client).await.uid;
    let member_id = get_test_member(&client).await;

    test_empty_documents(client.get_all_notes_for_member(&uid, &member_id).await);

    let note_id = test_add_document(client.add_note(&NoteCreateData {
        member: member_id.clone(),
        title: "Test Note".to_string(),
        note: "test".to_string(),
        markdown: None,
        color: "#ff0000".to_string(),
        date: time,
    }).await);

    test_get_documents(client.get_all_notes_for_member(&uid, &member_id).await, &note_id, |note| check_note(note, &member_id, time));
    test_get_document(client.get_note(&uid, &note_id).await, &note_id, |note| check_note(note, &member_id, time));

    test_document_action(client.update_note(&note_id, &NoteUpdateData {
        title: Some("Updated Note".to_string()),
        note: Some("updated".to_string()),
        markdown: None,
        color: Some("#00ff00".to_string()),
    }).await);

    test_get_document(client.get_note(&uid, &note_id).await, &note_id, |note| {
        assert_eq!(note.member, member_id);
        assert_eq!(note.title, "Updated Note");
        assert_eq!(note.note, "updated");
        assert_eq!(note.color, "#00ff00");
        assert_eq!(note.date, time);
    });

    test_document_action(client.delete_note(&note_id).await);
    cleanup_test_member(&client, &member_id).await;
}

fn check_note(note: &NoteData, member_id: &str, time: UnixTimestamp) {
    assert_eq!(note.member, member_id);
    assert_eq!(note.title, "Test Note");
    assert_eq!(note.note, "test");
    assert_eq!(note.color, "#ff0000");
    assert_eq!(note.date, time);
}
