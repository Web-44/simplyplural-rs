use serial_test::parallel;
use crate::tests::get_client;

#[tokio::test]
#[parallel]
async fn test_analytics() {
    let analytics = get_client().get_analytics(0, 9999999999999).await;
    let _ = analytics.expect("analytics");
}