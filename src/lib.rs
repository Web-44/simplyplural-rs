use reqwest::{Client, Error};
use std::fmt::Display;
use crate::error::SPError;

pub mod handler;
pub mod error;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

pub type SPResult<T> = Result<T, Error>;
pub type UnixTimestamp = i64;

const API_BASE_URL: &str = "https://api.apparyllis.com/v1";
const DEV_API_BASE_URL: &str = "https://devapi.apparyllis.com/v1";

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPEnvironment {
    /// Use the production API (https://api.apparyllis.com)
    /// The app is found at https://app.apparyllis.com
    Production,
    /// Use the development API (https://devapi.apparyllis.com)
    /// The app is found at https://devapp.apparyllis.com
    Development
}

#[derive(Debug, Clone)]
pub struct SPClient {
    pub(crate) http_client: Client,
    pub(crate) token: String,
    pub(crate) environment: SPEnvironment
}

impl SPClient {
    /// Creates a new SPClient instance.
    /// # Arguments
    /// * `token` - A string slice that holds the API token.
    /// * `devapi` - A boolean indicating whether to use the development API.
    pub fn new(token: impl ToString, environment: SPEnvironment) -> SPClient {
        let client = Client::new();

        Self::new_with_client(token, environment, client)
    }

    pub fn new_with_client(token: impl ToString, environment: SPEnvironment, client: Client) -> SPClient {
        let token = token.to_string();

        SPClient {
            http_client: client,
            token,
            environment
        }
    }

    pub fn environment(&self) -> SPEnvironment {
        self.environment
    }

    pub fn base_url(&self) -> &str {
        match self.environment {
            SPEnvironment::Production => API_BASE_URL,
            SPEnvironment::Development => DEV_API_BASE_URL,
        }
    }

    /// Get analytics of authenticated user
    /// # Arguments
    /// * `start_time` - Start time in unix timestamp (millisecond accuracy)
    /// * `end_time` - End time in unix timestamp (millisecond accuracy)
    #[cfg(feature = "analytics")]
    pub async fn get_analytics(&self, start_time: UnixTimestamp, end_time: UnixTimestamp) -> SPResult<handler::analytics::model::AnalyticsResponse> {
        handler::analytics::get_analytics(&self, start_time, end_time).await
    }

    /// Get fronters from the currently authenticated system
    #[cfg(feature = "front")]
    pub async fn get_current_fronters(&self) -> SPResult<Vec<handler::front::FrontResponse>> {
        handler::front::get_current_fronters(&self).await
    }

    /// Get Front History from a system between 2 timestamps
    /// # Arguments
    /// * `user_id` - The user ID of the system to get the front history from
    /// * `start_time` - Start time in unix timestamp (millisecond accuracy)
    /// * `end_time` - End time in unix timestamp (millisecond accuracy)
    #[cfg(feature = "front")]
    pub async fn get_front_history(&self, user_id: impl Display, start_time: UnixTimestamp, end_time: UnixTimestamp) -> SPResult<Vec<handler::front::FrontResponse>> {
        handler::front::get_front_history(&self, user_id, start_time, end_time).await
    }

    /// Get Front History for a specific member
    /// # Arguments
    /// * `member_id` - The member ID to get the front history for
    #[cfg(feature = "front")]
    pub async fn get_member_front_history(&self, member_id: impl Display) -> SPResult<Vec<handler::front::FrontResponse>> {
        handler::front::get_member_front_history(&self, member_id).await
    }

    /// Get Front History for a specific member between 2 timestamps
    /// # Arguments
    /// * `member_id` - The member ID to get the front history for
    /// * `start_time` - Start time in unix timestamp (millisecond accuracy)
    /// * `end_time` - End time in unix timestamp (millisecond accuracy)
    #[cfg(feature = "front")]
    pub async fn get_member_front_history_in_interval(&self, member_id: impl Display, start_time: UnixTimestamp, end_time: UnixTimestamp) -> SPResult<Vec<handler::front::FrontResponse>> {
        handler::front::get_member_front_history_in_interval(&self, member_id, start_time, end_time).await
    }

    /// Get a single Front History entry by Id
    /// # Arguments
    /// * `user_id` - The user ID of the system to get the front history from
    /// * `entry_id` - The entry ID of the front history entry to get
    #[cfg(feature = "front")]
    pub async fn get_front_history_entry(&self, user_id: impl Display, entry_id: impl Display) -> SPResult<handler::front::FrontResponse> {
        handler::front::get_front_history_entry(&self, user_id, entry_id).await
    }

    /// Add a Front history entry and returns its ID
    /// # Arguments
    /// * `entry` - The front history entry to add
    #[cfg(feature = "front")]
    pub async fn add_front_entry(&self, entry: &handler::front::model::FrontCreateData) -> Result<String, SPError> {
        handler::front::add_front_entry(&self, entry).await
    }

    /// Update a Front history entry by its ID
    /// # Arguments
    /// * `entry_id` - The entry ID of the front history entry to update
    /// * `entry` - The front history entry data to update
    #[cfg(feature = "front")]
    pub async fn update_front_entry(&self, entry_id: impl Display, entry: &handler::front::model::FrontUpdateData) -> SPResult<()> {
        handler::front::update_front_entry(&self, entry_id, entry).await
    }

    /// Delete a Front history entry by its ID
    /// # Arguments
    /// * `entry_id` - The entry ID of the front history entry to delete
    #[cfg(feature = "front")]
    pub async fn delete_front_entry(&self, entry_id: impl Display) -> SPResult<()> {
        handler::front::delete_front_entry(&self, entry_id).await
    }

    /// Convenience function to add a member to the front
    /// # Arguments
    /// * `member_id` - The member ID to add to the front
    /// * `start_time` - Start time in unix timestamp (millisecond accuracy)
    /// * `custom_status` - Optional custom status to set for the front entry
    #[cfg(feature = "front")]
    pub async fn add_member_to_front(&self, member_id: impl ToString, start_time: UnixTimestamp, custom_status: Option<String>) -> SPResult<String> {
        self.add_front_entry(&handler::front::model::FrontCreateData {
            member: member_id.to_string(),
            custom: false,
            live: true,
            start_time,
            end_time: None,
            custom_status
        }).await.map_err(|err| err.unwrap_reqwest())
    }

    /// Convenience function to remove a member from the front
    /// # Arguments
    /// * `entry_id` - The entry ID of the front history entry to update
    /// * `end_time` - End time in unix timestamp (millisecond accuracy)
    #[cfg(feature = "front")]
    pub async fn remove_member_from_front(&self, entry_id: impl Display, end_time: UnixTimestamp) -> SPResult<()> {
        self.update_front_entry(entry_id, &handler::front::model::FrontUpdateData {
            live: Some(false),
            end_time: Some(end_time),
            ..Default::default()
        }).await
    }

    document_methods!(automated_timer, "automated_timers", automated_timers,
        handler::automated_timers::AutomatedTimerResponse,
        handler::automated_timers::model::AutomatedTimerCreateData,
        handler::automated_timers::model::AutomatedTimerCreateData);
    document_methods_all!(automated_timers, "automated_timers", automated_timers,
        handler::automated_timers::AutomatedTimerResponse);

    document_methods!(member, "members", members,
        handler::members::MemberResponse,
        handler::members::model::MemberCreateData,
        handler::members::model::MemberCreateData);
    document_methods_all!(members, "members", members,
        handler::members::MemberResponse);

    document_methods!(note, "notes", notes,
        handler::notes::NoteResponse,
        handler::notes::model::NoteCreateData,
        handler::notes::model::NoteUpdateData);
    document_methods_for!(notes, member, "notes", notes,
        handler::notes::NoteResponse);

    document_methods!(poll, "polls", polls,
        handler::polls::PollResponse,
        handler::polls::model::PollCreateData,
        handler::polls::model::PollCreateData);
    document_methods_all!(polls, "polls", polls,
        handler::polls::PollResponse);

    document_methods!(repeated_timer, "repeated_timers", repeated_timers,
        handler::repeated_timers::RepeatedTimerResponse,
        handler::repeated_timers::model::RepeatedTimerCreateData,
        handler::repeated_timers::model::RepeatedTimerCreateData);
    document_methods_all!(repeated_timers, "repeated_timers", repeated_timers,
        handler::repeated_timers::RepeatedTimerResponse);

    /// Get the user associated with your currently authed token
    #[cfg(feature = "users")]
    pub async fn get_self_user(&self) -> SPResult<handler::users::UserResponse> {
        handler::users::get_self_user(&self).await
    }

    /// Get a user by Id
    /// # Arguments
    /// * `user_id` - Id of the user
    #[cfg(feature = "users")]
    pub async fn get_user(&self, user_id: impl Display) -> SPResult<handler::users::UserResponse> {
        handler::users::get_user(&self, user_id).await
    }

    /// Update your own user
    /// # Arguments
    /// * `user` - The user data to update
    #[cfg(feature = "users")]
    pub async fn update_user(&self, user: &handler::users::model::UserData) -> SPResult<()> {
        handler::users::update_user(&self, &user.uid, user).await
    }

    /// Set your own username
    /// # Arguments
    /// * `user_id` - Id of the user
    /// * `username` - The new username to set
    #[cfg(feature = "users")]
    pub async fn set_username(&self, user_id: impl Display, username: impl ToString) -> SPResult<handler::users::UsernameResponse> {
        handler::users::set_username(&self, user_id, username).await
    }
}