use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_messages::{Message, Messages};

use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "protected.html")]
struct ProtectedTemplate<'a> {
    messages: Vec<Message>,
    username: &'a str,
}

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user().await {
            Some(user) => Html(
                ProtectedTemplate {
                    messages: messages.into_iter().collect(),
                    username: &user.username,
                }
                .render()
                .unwrap(),
            )
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
