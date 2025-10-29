use crate::handler::automated_timers::model::{AutomatedTimerAction, AutomatedTimerCreateData, AutomatedTimerData};
use crate::tests::{get_client, get_self_user, test_add_document, test_document_action, test_empty_documents, test_get_document, test_get_documents};
use serial_test::parallel;

#[cfg(feature = "users")]
#[tokio::test]
#[parallel]
async fn test_automated_timers() {
    let client = get_client();
    let uid = get_self_user(&client).await.uid;

    test_empty_documents(client.get_all_automated_timers(&uid).await);

    let mut timer_data = AutomatedTimerCreateData {
        name: "Test Timer".to_string(),
        message: "Test Message".to_string(),
        delay: 0,
        r#type: AutomatedTimerAction::MemberFrontChange,
    };
    let timer_id = test_add_document(client.add_automated_timer(&timer_data).await);

    test_get_documents(client.get_all_automated_timers(&uid).await, &timer_id, |timer| check_timer(timer, 0, AutomatedTimerAction::MemberFrontChange));
    test_get_document(client.get_automated_timer(&uid, &timer_id).await, &timer_id, |timer| check_timer(timer, 0, AutomatedTimerAction::MemberFrontChange));

    timer_data.delay = 3;
    timer_data.r#type = AutomatedTimerAction::CustomFrontChange;
    test_document_action(client.update_automated_timer(&timer_id, &timer_data).await);
    test_get_document(client.get_automated_timer(&uid, &timer_id).await, &timer_id, |timer| check_timer(timer, 3, AutomatedTimerAction::CustomFrontChange));

    test_document_action(client.delete_automated_timer(&timer_id).await);
}

fn check_timer(timer: &AutomatedTimerData, delay: u32, r#type: AutomatedTimerAction) {
    assert_eq!(timer.name, "Test Timer");
    assert_eq!(timer.message, "Test Message");
    assert_eq!(timer.delay, delay);
    assert_eq!(timer.r#type, r#type);
}