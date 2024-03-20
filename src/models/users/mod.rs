use async_trait::async_trait;
use chrono::Utc;
use loco_oauth2::models::users::OAuth2UserTrait;
use loco_rs::{
    hash,
    model::{ModelError, ModelResult},
    prelude::*,
};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, QueryFilter, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::_entities::{o_auth2_sessions, prelude::*, users};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OAuth2UserProfile {
    // https://www.googleapis.com/auth/userinfo.email See your primary Google Account email address
    pub email: String,
    // Fields below
    // https://www.googleapis.com/auth/userinfo.profile See your personal info, including any personal info you've made publicly available
    pub name: String,
    pub sub: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email_verified: bool,
    pub locale: String,
}

#[async_trait]
impl OAuth2UserTrait<OAuth2UserProfile> for users::Model {
    /// Asynchronously finds user by OAuth2 session id.
    /// # Arguments
    /// * `db` - Database connection
    /// * `cookie` - OAuth2 session id
    ///
    /// # Returns
    /// * `Self` - The `OAuth2UserTrait` struct
    ///
    /// # Errors
    /// * `ModelError` - When could not find the user in the DB
    async fn find_by_oauth2_session_id(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> ModelResult<Self> {
        // find the session by the session id
        let session = OAuth2Sessions::find()
            .filter(o_auth2_sessions::Column::SessionId.eq(session_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        // if the session is found, find the user by the user id
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(session.user_id))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    /// Asynchronously upsert user with OAuth data and saves it to the
    /// database.
    /// # Arguments
    /// * `db` - Database connection
    /// * `profile` - OAuth profile
    ///
    /// # Returns
    /// * `Self` - The `OAuth2UserTrait` struct
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    async fn upsert_with_oauth(
        db: &DatabaseConnection,
        profile: &OAuth2UserProfile,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let user = match Users::find()
            .filter(users::Column::Email.eq(&profile.email))
            .one(&txn)
            .await?
        {
            None => {
                // We use the sub field as the user fake password since sub is unique
                let password_hash =
                    hash::hash_password(&profile.sub).map_err(|e| ModelError::Any(e.into()))?;
                // Create the user into the database
                users::ActiveModel {
                    pid: ActiveValue::set(uuid::Uuid::new_v4()),
                    email: ActiveValue::set(profile.email.to_string()),
                    name: ActiveValue::set(profile.name.to_string()),
                    email_verified_at: ActiveValue::set(Some(Utc::now().naive_utc())),
                    password: ActiveValue::set(password_hash),
                    ..Default::default()
                }
                .insert(&txn)
                .await
                .map_err(|e| {
                    tracing::error!("Error while trying to create user: {e}");
                    ModelError::Any(e.into())
                })?
            }
            // Do nothing if user exists
            Some(user) => user,
        };
        txn.commit().await?;
        Ok(user)
    }
}

impl users::Model {
    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// When could not convert user claims to JWT token
    pub fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        Ok(
            loco_rs::auth::jwt::JWT::new(secret)
                .generate_token(expiration, self.pid.to_string())?,
        )
    }
}
