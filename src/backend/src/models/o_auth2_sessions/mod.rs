use async_trait::async_trait;
use chrono::Utc;
use loco_oauth2::{
    TokenResponse, basic::BasicTokenResponse, models::oauth2_sessions::OAuth2SessionsTrait,
};
use loco_rs::{
    model::{ModelError, ModelResult},
    prelude::*,
};
use sea_orm::{ActiveValue, TransactionTrait, entity::prelude::*};

use crate::models::_entities::{o_auth2_sessions, users};

#[async_trait]
impl OAuth2SessionsTrait<users::Model> for o_auth2_sessions::Model {
    /// Check if a session is expired from the database
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// session_id: &str - Session id
    /// # Returns
    /// A boolean
    /// # Errors
    /// Returns a `ModelError` if the session is not found
    async fn is_expired(db: &DatabaseConnection, session_id: &str) -> ModelResult<bool> {
        let oauth2_session = o_auth2_sessions::Entity::find()
            .filter(o_auth2_sessions::Column::SessionId.eq(session_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        Ok(oauth2_session.expires_at < Utc::now().naive_utc())
    }

    /// Upsert a session with OAuth
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// token: &`BasicTokenResponse` - OAuth token
    /// user: &`users::Model` - User
    /// # Returns
    /// A session
    /// # Errors
    /// Returns a `ModelError` if the session cannot be upserted
    async fn upsert_with_oauth2(
        db: &DatabaseConnection,
        token: &BasicTokenResponse,
        user: &users::Model,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let oauth2_session_id = token.access_token().secret().clone();
        let oauth2_session = match o_auth2_sessions::Entity::find()
            .filter(o_auth2_sessions::Column::UserId.eq(user.id))
            .one(&txn)
            .await?
        {
            Some(oauth2_session) => {
                // Update the session
                let mut oauth2_session: o_auth2_sessions::ActiveModel = oauth2_session.into();
                oauth2_session.session_id = ActiveValue::set(oauth2_session_id);
                oauth2_session.expires_at =
                    ActiveValue::set(Utc::now().naive_utc() + token.expires_in().unwrap());
                oauth2_session.updated_at = ActiveValue::set(Utc::now().naive_utc());
                oauth2_session.update(&txn).await?
            }
            None => {
                // Create the session
                o_auth2_sessions::ActiveModel {
                    session_id: ActiveValue::set(oauth2_session_id),
                    expires_at: ActiveValue::set(
                        Utc::now().naive_utc() + token.expires_in().unwrap(),
                    ),
                    user_id: ActiveValue::set(user.id),
                    ..Default::default()
                }
                .insert(&txn)
                .await?
            }
        };
        txn.commit().await?;
        Ok(oauth2_session)
    }
}
