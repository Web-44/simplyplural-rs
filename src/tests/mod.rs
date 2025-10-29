#[cfg(feature = "analytics")]
mod analytics;
#[cfg(feature = "automated_timers")]
mod automated_timers;
#[cfg(feature = "front")]
mod front;
#[cfg(feature = "members")]
mod members;
#[cfg(feature = "notes")]
mod notes;
#[cfg(feature = "polls")]
mod polls;
#[cfg(feature = "repeated_timers")]
mod repeated_timers;
#[cfg(feature = "users")]
mod users;

use std::env::var;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use crate::{SPClient, SPEnvironment, SPResult};

static MEMBER_CLEANUP_REQUIRED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));

pub fn get_client() -> SPClient {
    let token = var("TOKEN").expect("TOKEN environment variable not set to valid SP token");
    SPClient::new(token, SPEnvironment::Development)
}

#[cfg(feature = "users")]
pub async fn get_self_user(client: &SPClient) -> crate::handler::users::model::UserData {
    let self_user = client.get_self_user().await.expect("self-user");
    assert!(self_user.exists);

    let user_data = self_user.content.expect("self-user data");
    assert_eq!(user_data.uid, self_user.id); // for users the uid is the same as id

    user_data
}

#[cfg(feature = "members")]
pub async fn get_test_member(client: &SPClient) -> String {
    let mut should_cleanup = MEMBER_CLEANUP_REQUIRED.lock().await;
    if *should_cleanup {
        *should_cleanup = false;
        let members = client.get_all_members(&get_self_user(client).await.uid).await;
        let members = members.expect("members");
        if !members.is_empty() {
            //Cleanup old test members if any exist
            for member in members {
                if let Some(content) = member.content {
                    if content.name == "test" || content.name == "updated" {
                        let res = client.delete_member(&member.id).await;
                        assert!(res.is_ok());
                    }
                }
            }
        }
    }

    let member_data = crate::handler::members::model::MemberCreateData {
        name: "test".to_string(),
        description: Some("test member".to_string()),
        description_markdown: None,
        pronouns: Some("they/them".to_string()),
        pluralkit_id: None,
        avatar: None,
        private: None,
        prevent_trusted: None,
        prevents_front_notifications: None,
        color: Some("#00ffff".to_string()),
        info: None,
    };
    let member_id = client.add_member(&member_data).await;
    member_id.expect("add member")
}

#[cfg(feature = "members")]
pub async fn cleanup_test_member(client: &SPClient, member_id: &str) {
    let res = client.delete_member(member_id).await;
    assert!(res.is_ok());
}

pub fn test_add_document(result: SPResult<String>) -> String {
    let id = result.expect("add document");

    assert!(!id.is_empty());
    assert!(!id.contains(" "));

    id
}

pub fn test_get_documents<F, T>(result: SPResult<Vec<crate::handler::model::IdExists<T>>>, id: &str, check: F) -> T
where
    F: Fn(&T) {
    let docs = result.expect("get documents");
    assert_eq!(docs.len(), 1);

    let doc = docs.into_iter().next().unwrap();
    assert!(doc.exists);
    assert_eq!(doc.id, id);

    let data = doc.content.expect("document content");
    check(&data);

    data
}

pub fn test_get_document<F, T>(result: SPResult<crate::handler::model::IdExists<T>>, id: &str, check: F) -> T
where
    F: Fn(&T) {
    let doc = result.expect("get document");
    assert!(doc.exists);
    assert_eq!(doc.id, id);

    let data = doc.content.expect("document content");
    check(&data);

    data
}

pub fn test_document_action(result: SPResult<()>) {
    assert!(result.is_ok());
}

pub fn test_empty_documents<T>(result: SPResult<Vec<crate::handler::model::IdExists<T>>>) {
    let docs = result.expect("empty documents");
    assert!(docs.is_empty());
}