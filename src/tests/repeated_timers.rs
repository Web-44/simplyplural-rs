use crate::handler::repeated_timers::model::{RepeatedTimerCreateData, RepeatedTimerData, RepeatedTimerStartTime, RepeatedTimerTime};
use crate::tests::{get_client, get_self_user, test_add_document, test_document_action, test_empty_documents, test_get_document, test_get_documents};
use serial_test::parallel;

#[cfg(feature = "users")]
#[tokio::test]
#[parallel]
async fn test_repeated_timers() {
    let client = get_client();
    let uid = get_self_user(&client).await.uid;

    test_empty_documents(client.get_all_repeated_timers(&uid).await);

    let mut timer_data = RepeatedTimerCreateData {
        name: "Test Timer".to_string(),
        message: "Test Message".to_string(),
        day_interval: 14,
        time: RepeatedTimerTime {
            hour: 11,
            minute: 0
        },
        start_time: RepeatedTimerStartTime {
            year: 2024,
            month: 11,
            day: 1
        }
    };
    let timer_id = test_add_document(client.add_repeated_timer(&timer_data).await);

    test_get_documents(client.get_all_repeated_timers(&uid).await, &timer_id, |timer| check_timer(timer, 14));
    test_get_document(client.get_repeated_timer(&uid, &timer_id).await, &timer_id, |timer| check_timer(timer, 14));

    timer_data.day_interval = 16;
    test_document_action(client.update_repeated_timer(&timer_id, &timer_data).await);
    test_get_document(client.get_repeated_timer(&uid, &timer_id).await, &timer_id, |timer| check_timer(timer, 16));

    test_document_action(client.delete_repeated_timer(&timer_id).await);
}

fn check_timer(timer: &RepeatedTimerData, interval: u32) {
    assert_eq!(timer.name, "Test Timer");
    assert_eq!(timer.message, "Test Message");
    assert_eq!(timer.day_interval, interval);

    assert_eq!(timer.time.hour, 11);
    assert_eq!(timer.time.minute, 0);

    assert_eq!(timer.start_time.year, 2024);
    assert_eq!(timer.start_time.month, 11);
    assert_eq!(timer.start_time.day, 1);
}