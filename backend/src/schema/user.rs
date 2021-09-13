use async_graphql::{Context, Object, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  models::UserSession,
  repositories::UserSessionRepository,
  services::{hour_stats::HourStatsService, user::UserService},
  utils::{MAX_DATETIME, MIN_DATETIME},
};

use super::{HubbitSchemaError, HubbitSchemaResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Uuid,
}

#[Object]
impl User {
  async fn id(&self) -> Uuid {
    self.id
  }

  async fn nick(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.nick)
  }

  async fn first_name(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.first_name)
  }

  async fn last_name(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.last_name)
  }

  async fn avatar_url(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.avatar_url)
  }

  async fn groups(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<String>> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(
      user
        .groups
        .into_iter()
        .map(|group| group.super_group.name)
        .collect(),
    )
  }

  async fn hour_stats(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<u32>> {
    let hour_stats_service = context.data_unchecked::<HourStatsService>();
    Ok(
      hour_stats_service
        .get_for_user(self.id)
        .await
        .map_err(|_| HubbitSchemaError::InternalError)?,
    )
  }

  async fn recent_sessions(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<Session>> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let sessions = user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(
      sessions
        .iter()
        .map(|session| Session {
          start_time: session.start_time,
          end_time: session.end_time,
        })
        .take(10)
        .collect(),
    )
  }

  async fn longest_session(&self, context: &Context<'_>) -> HubbitSchemaResult<Option<Session>> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let sessions = user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    let mut longest_session: Option<UserSession> = None;
    for session in sessions {
      if let Some(longest_session_inner) = &longest_session {
        if session.end_time - session.start_time
          > longest_session_inner.end_time - longest_session_inner.start_time
        {
          longest_session = Some(session);
        }
      } else {
        longest_session = Some(session);
      }
    }

    Ok(longest_session.map(|session| Session {
      start_time: session.start_time,
      end_time: session.end_time,
    }))
  }
}

#[derive(SimpleObject)]
pub struct Session {
  start_time: DateTime<Utc>,
  end_time: DateTime<Utc>,
}
