use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{interval, Duration as TokioDuration};

use crate::time_parser::Duration;

/// Session status enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "error")]
    Error,
}

/// Sleep prevention session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepSession {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub duration: Duration,
    pub status: SessionStatus,
    pub created_by: String,
}

impl SleepSession {
    /// Create a new sleep session
    pub fn new(duration: Duration) -> Self {
        let session_id = format!(
            "sleep_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );

        Self {
            id: session_id,
            start_time: Utc::now(),
            duration,
            status: SessionStatus::Active,
            created_by: "system".to_string(),
        }
    }

    /// Calculate remaining time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        if self.status != SessionStatus::Active {
            return 0;
        }

        let elapsed = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds();

        if elapsed < 0 {
            return self.duration.total_seconds;
        }

        let elapsed = elapsed as u64;
        if elapsed >= self.duration.total_seconds {
            0
        } else {
            self.duration.total_seconds.saturating_sub(elapsed)
        }
    }

    /// Check if session has expired
    pub fn is_expired(&self) -> bool {
        self.status == SessionStatus::Active && self.remaining_seconds() == 0
    }

    /// Cancel the session
    pub fn cancel(&mut self) {
        self.status = SessionStatus::Cancelled;
    }

    /// Mark session as expired
    pub fn expire(&mut self) {
        self.status = SessionStatus::Expired;
    }

    /// Mark session as error
    pub fn error(&mut self) {
        self.status = SessionStatus::Error;
    }

    /// Get progress percentage (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.status != SessionStatus::Active {
            return if self.status == SessionStatus::Expired {
                1.0
            } else {
                0.0
            };
        }

        let elapsed = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds();

        if elapsed <= 0 {
            return 0.0;
        }

        let progress = elapsed as f64 / self.duration.total_seconds as f64;
        progress.min(1.0)
    }
}

/// Thread-safe session manager
#[derive(Debug)]
pub struct SessionManager {
    current_session: Arc<Mutex<Option<SleepSession>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            current_session: Arc::new(Mutex::new(None)),
        }
    }

    /// Start a new sleep prevention session
    pub fn start_session(&self, duration: &Duration) -> Result<SleepSession, SessionError> {
        let mut session_lock = self
            .current_session
            .lock()
            .map_err(|_| SessionError::LockError)?;

        // Check if there's already an active session
        if let Some(ref session) = *session_lock {
            if session.status == SessionStatus::Active {
                return Err(SessionError::SessionAlreadyActive(session.id.clone()));
            }
        }

        let session = SleepSession::new(duration.clone());
        *session_lock = Some(session.clone());

        Ok(session)
    }

    /// Stop the current session
    pub fn stop_session(&self) -> Result<Option<SleepSession>, SessionError> {
        let mut session_lock = self
            .current_session
            .lock()
            .map_err(|_| SessionError::LockError)?;

        if let Some(ref mut session) = *session_lock {
            if session.status == SessionStatus::Active {
                session.cancel();
                return Ok(Some(session.clone()));
            }
        }

        Ok(None)
    }

    /// Get current session status
    pub fn get_current_session(&self) -> Result<Option<SleepSession>, SessionError> {
        let session_lock = self
            .current_session
            .lock()
            .map_err(|_| SessionError::LockError)?;

        Ok(session_lock.clone())
    }

    /// Update session status (for expiration, errors)
    pub fn update_session_status(&self, status: SessionStatus) -> Result<(), SessionError> {
        let mut session_lock = self
            .current_session
            .lock()
            .map_err(|_| SessionError::LockError)?;

        if let Some(ref mut session) = *session_lock {
            match status {
                SessionStatus::Expired => session.expire(),
                SessionStatus::Error => session.error(),
                SessionStatus::Cancelled => session.cancel(),
                _ => {}
            }
        }

        Ok(())
    }

    /// Check and handle session expiration
    pub fn check_expiration(&self) -> Result<bool, SessionError> {
        let mut session_lock = self
            .current_session
            .lock()
            .map_err(|_| SessionError::LockError)?;

        if let Some(ref mut session) = *session_lock {
            if session.is_expired() {
                session.expire();
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Clear completed/cancelled sessions
    pub fn cleanup_session(&self) -> Result<(), SessionError> {
        let mut session_lock = self
            .current_session
            .lock()
            .map_err(|_| SessionError::LockError)?;

        if let Some(ref session) = *session_lock {
            match session.status {
                SessionStatus::Expired | SessionStatus::Cancelled | SessionStatus::Error => {
                    *session_lock = None;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Start automatic session monitoring
    pub fn start_monitoring(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(1));

            loop {
                interval.tick().await;

                // Check for expiration
                if let Ok(expired) = self.check_expiration() {
                    if expired {
                        log::info!("Sleep prevention session expired");
                        // TODO: Emit event to frontend
                    }
                }

                // Cleanup completed sessions after 5 seconds
                tokio::time::sleep(TokioDuration::from_secs(5)).await;
                let _ = self.cleanup_session();
            }
        })
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Session management errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionError {
    SessionAlreadyActive(String),
    NoActiveSession,
    LockError,
    InvalidSession,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::SessionAlreadyActive(id) => {
                write!(
                    f,
                    "A sleep prevention session is already active (ID: {})",
                    id
                )
            }
            SessionError::NoActiveSession => {
                write!(f, "No active sleep prevention session")
            }
            SessionError::LockError => {
                write!(f, "Failed to acquire session lock")
            }
            SessionError::InvalidSession => {
                write!(f, "Invalid session state")
            }
        }
    }
}

impl std::error::Error for SessionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_parser::Duration;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_session_creation() {
        let duration = Duration::new(1, 30, 0).unwrap();
        let session = SleepSession::new(duration);

        assert_eq!(session.status, SessionStatus::Active);
        assert!(session.remaining_seconds() > 5390); // Should be close to 5400 seconds
    }

    #[test]
    fn test_session_manager() {
        let manager = SessionManager::new();
        let duration = Duration::new(0, 5, 0).unwrap();

        // Start session
        let session = manager.start_session(&duration).unwrap();
        assert_eq!(session.status, SessionStatus::Active);

        // Try to start another (should fail)
        assert!(manager.start_session(&duration).is_err());

        // Stop session
        let stopped = manager.stop_session().unwrap();
        assert!(stopped.is_some());
    }

    #[test]
    fn test_session_expiration() {
        let duration = Duration::new(0, 1, 0).unwrap(); // 1 minute (minimum)
        let session = SleepSession::new(duration);

        // Wait for expiration (in real test, you'd mock time)
        std::thread::sleep(StdDuration::from_millis(1100));
        // Note: This test would need time mocking for reliable testing
        // For now, just test the is_expired logic
        assert!(!session.is_expired()); // Should not be expired after 1 second
    }
}
