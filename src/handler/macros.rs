macro_rules! document_handler {
    ($name:ident, $url:literal, $response:ty, $create_data:ty, $update_data:ty) => {
        use std::fmt::Display;
        use crate::{SPClient, SPResult};
        use crate::handler::request::{delete_without_response, get, patch_without_response, post_clear};

        paste::paste! {
            pub(crate) async fn [<get_ $name>](client: &SPClient, user_id: impl Display, [<$name _id>]: impl Display) -> SPResult<$response> {
                get(client, format!("/{}/{}/{}", stringify!($name), user_id, [<$name _id>])).await
            }

            pub(crate) async fn [<add_ $name>](client: &SPClient, [<$name _data>]: &$create_data) -> SPResult<String> {
                post_clear(client, format!("/{}", stringify!($name)), [<$name _data>], true).await
            }

            pub(crate) async fn [<update_ $name>](client: &SPClient, [<$name _id>]: impl Display, [<$name _data>]: &$update_data) -> SPResult<()> {
                patch_without_response(client, format!("/{}/{}", stringify!($name), [<$name _id>]), [<$name _data>]).await
            }
            
            pub(crate) async fn [<delete_ $name>](client: &SPClient, [<$name _id>]: impl Display) -> SPResult<()> {
                delete_without_response(client, format!("/{}/{}", stringify!($name), [<$name _id>])).await
            }
        }
    };
}

macro_rules! document_handler_all {
    ($name:ident, $url:literal, $response:ty) => {
        paste::paste! {
            pub(crate) async fn [<get_all_ $name>](client: &SPClient, user_id: impl Display) -> SPResult<Vec<$response>> {
                get(client, format!("/{}/{}", stringify!($url), user_id)).await
            }
        }
    };
}

macro_rules! document_handler_for {
    ($name:ident, $url:literal, $for_name:ident, $response:ty) => {
        paste::paste! {
            pub(crate) async fn [<get_all_ $name _for_ $for_name>](client: &SPClient, user_id: impl Display, [<$for_name _id>]: impl Display) -> SPResult<Vec<$response>> {
                get(client, format!("/{}/{}/{}", stringify!($url), user_id, [<$for_name _id>])).await
            }
        }
    };
}