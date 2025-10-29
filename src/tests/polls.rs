use std::time::{SystemTime, UNIX_EPOCH};
use serial_test::parallel;
use crate::handler::polls::model::{PollCreateData, PollData, PollVote};
use crate::tests::{cleanup_test_member, get_client, get_self_user, get_test_member, test_add_document, test_document_action, test_empty_documents, test_get_document, test_get_documents};
use crate::UnixTimestamp;

#[cfg(all(feature = "members", feature = "users"))]
#[tokio::test]
#[parallel]
async fn test_polls() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_millis();
    let poll_time = time as UnixTimestamp + 60000;

    let client = get_client();
    let uid = get_self_user(&client).await.uid;

    test_empty_documents(client.get_all_polls(&uid).await);

    let mut poll_data = PollCreateData {
        name: "Test Poll".to_string(),
        description: "Test Poll".to_string(),
        custom: false,
        end_time: poll_time,
        allow_abstain: Some(true),
        allow_veto: Some(true),
        options: None,
        votes: None,
    };
    let poll_id = test_add_document(client.add_poll(&poll_data).await);

    test_get_documents(client.get_all_polls(&uid).await, &poll_id, |poll| check_poll(poll, poll_time, None));
    test_get_document(client.get_poll(&uid, &poll_id).await, &poll_id, |poll| check_poll(poll, poll_time, None));

    let member_id = get_test_member(&client).await;

    poll_data.votes = Some(vec![
        PollVote {
            id: member_id.clone(),
            vote: "abstain".to_string(),
            comment: "test".to_string(),
        }
    ]);
    test_document_action(client.update_poll(&poll_id, &poll_data).await);
    test_get_document(client.get_poll(&uid, &poll_id).await, &poll_id, |poll| check_poll(poll, poll_time, Some(member_id.clone())));

    test_document_action(client.delete_poll(&poll_id).await);

    cleanup_test_member(&client, &member_id).await;
}

fn check_poll(poll: &PollData, poll_time: UnixTimestamp, vote_member_id: Option<String>) {
    assert_eq!(poll.name, "Test Poll");
    assert_eq!(poll.description, "Test Poll");
    assert!(!poll.custom);
    assert_eq!(poll.end_time, poll_time);
    assert_eq!(poll.allow_abstain, Some(true));
    assert_eq!(poll.allow_veto, Some(true));
    assert!(poll.options.is_none());

    if let Some(vote_member_id) = vote_member_id {
        let votes = poll.votes.as_ref().expect("poll votes");
        assert_eq!(votes.len(), 1);
        let vote = &votes[0];
        assert_eq!(vote.id, vote_member_id);
        assert_eq!(vote.vote, "abstain");
        assert_eq!(vote.comment, "test");
    } else {
        assert!(poll.votes.is_none());
    }
}