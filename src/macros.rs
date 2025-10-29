macro_rules! document_methods {
    ($name:ident, $feature:literal, $handler:ident, $response:ty, $create_data:ty, $update_data:ty) => {
        paste::paste! {
            #[cfg(feature = $feature)]
            pub async fn [<get_ $name>](&self, user_id: impl Display, [<$name _id>]: impl Display) -> SPResult<$response> {
                handler::$handler::[<get_ $name>](&self, user_id, [<$name _id>]).await
            }

            #[cfg(feature = $feature)]
            pub async fn [<add_ $name>](&self, [<$name _data>]: &$create_data) -> SPResult<String> {
                handler::$handler::[<add_ $name>](&self, [<$name _data>]).await
            }

            #[cfg(feature = $feature)]
            pub async fn [<update_ $name>](&self, [<$name _id>]: impl Display, [<$name _data>]: &$update_data) -> SPResult<()> {
                handler::$handler::[<update_ $name>](&self, [<$name _id>], [<$name _data>]).await
            }

            #[cfg(feature = $feature)]
            pub async fn [<delete_ $name>](&self, [<$name _id>]: impl Display) -> SPResult<()> {
                handler::$handler::[<delete_ $name>](&self, [<$name _id>]).await
            }
        }
    };
}

macro_rules! document_methods_all {
    ($name:ident, $feature:literal, $handler:ident, $response:ty) => {
        paste::paste! {
            #[cfg(feature = $feature)]
            pub async fn [<get_all_ $name>](&self, user_id: impl Display) -> SPResult<Vec<$response>> {
                handler::$handler::[<get_all_ $name>](&self, user_id).await
            }
        }
    };
}

macro_rules! document_methods_for {
    ($name:ident, $for_name:ident, $feature:literal, $handler:ident, $response:ty) => {
        paste::paste! {
            #[cfg(feature = $feature)]
            pub async fn [<get_all_ $name _for_ $for_name>](&self, user_id: impl Display, [<$for_name _id>]: impl Display) -> SPResult<Vec<$response>> {
                handler::$handler::[<get_all_ $name _for_ $for_name>](&self, user_id, [<$for_name _id>]).await
            }
        }
    };
}