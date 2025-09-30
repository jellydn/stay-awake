use serde::{Deserialize, Serialize};
use std::fmt;

/// Duration representation with validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub total_seconds: u64,
}

impl Duration {
    /// Create a new Duration with validation
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Result<Self, DurationError> {
        // Validate ranges
        if hours > 8 {
            return Err(DurationError::ExceedsMaximum(8));
        }
        if minutes >= 60 {
            return Err(DurationError::InvalidMinutes(minutes));
        }
        if seconds >= 60 {
            return Err(DurationError::InvalidSeconds(seconds));
        }

        let total_seconds = (hours as u64 * 3600) + (minutes as u64 * 60) + (seconds as u64);

        // Must be at least 1 minute
        if total_seconds < 60 {
            return Err(DurationError::TooShort);
        }

        // Must not exceed 8 hours (28800 seconds)
        if total_seconds > 28800 {
            return Err(DurationError::ExceedsMaximum(8));
        }

        Ok(Duration {
            hours,
            minutes,
            seconds,
            total_seconds,
        })
    }

    /// Parse duration from string formats
    /// Supports: "2h 30m", "150m", "2:30", "2.5h"
    pub fn parse(input: &str) -> Result<Self, DurationError> {
        let input = input.trim().to_lowercase();

        // Handle "2h 30m" or "2h30m" format
        if input.contains('h') || input.contains('m') {
            return Self::parse_hours_minutes(&input);
        }

        // Handle "2:30" format
        if input.contains(':') {
            return Self::parse_colon_format(&input);
        }

        // Handle decimal hours like "2.5h" (already covered above)
        // Handle pure numbers as minutes
        if let Ok(minutes) = input.parse::<u32>() {
            return Self::new(0, minutes, 0);
        }

        Err(DurationError::InvalidFormat(input.to_string()))
    }

    fn parse_hours_minutes(input: &str) -> Result<Self, DurationError> {
        let mut hours = 0;
        let mut minutes = 0;

        // Extract hours if present
        if let Some(h_pos) = input.find('h') {
            let h_str = &input[..h_pos];
            if let Ok(h) = h_str.parse::<f32>() {
                hours = h as u32;
                // Handle decimal hours (e.g., "2.5h")
                let fractional_minutes = ((h - hours as f32) * 60.0) as u32;
                minutes += fractional_minutes;
            }
        }

        // Extract minutes if present
        if let Some(m_pos) = input.find('m') {
            // Find start of minutes number by looking backward from 'm'
            let mut start = m_pos;
            for (i, c) in input[..m_pos].char_indices().rev() {
                if c.is_ascii_digit() || c == '.' {
                    start = i;
                } else {
                    break;
                }
            }

            if start < m_pos {
                let m_str = &input[start..m_pos];
                if !m_str.contains('h') {
                    // Don't re-parse hours
                    if let Ok(m) = m_str.parse::<u32>() {
                        minutes += m;
                    }
                }
            }
        }

        Self::new_with_conversion(hours, minutes, 0)
    }

    /// Create Duration with automatic hour/minute conversion
    fn new_with_conversion(
        mut hours: u32,
        mut minutes: u32,
        seconds: u32,
    ) -> Result<Self, DurationError> {
        // Convert excess minutes to hours
        if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }

        Self::new(hours, minutes, seconds)
    }

    fn parse_colon_format(input: &str) -> Result<Self, DurationError> {
        let parts: Vec<&str> = input.split(':').collect();

        match parts.len() {
            2 => {
                let hours = parts[0]
                    .parse::<u32>()
                    .map_err(|_| DurationError::InvalidFormat(input.to_string()))?;
                let minutes = parts[1]
                    .parse::<u32>()
                    .map_err(|_| DurationError::InvalidFormat(input.to_string()))?;
                Self::new_with_conversion(hours, minutes, 0)
            }
            3 => {
                let hours = parts[0]
                    .parse::<u32>()
                    .map_err(|_| DurationError::InvalidFormat(input.to_string()))?;
                let minutes = parts[1]
                    .parse::<u32>()
                    .map_err(|_| DurationError::InvalidFormat(input.to_string()))?;
                let seconds = parts[2]
                    .parse::<u32>()
                    .map_err(|_| DurationError::InvalidFormat(input.to_string()))?;
                Self::new_with_conversion(hours, minutes, seconds)
            }
            _ => Err(DurationError::InvalidFormat(input.to_string())),
        }
    }

    /// Format duration as human-readable string
    pub fn format(&self) -> String {
        if self.hours > 0 {
            format!("{}h {}m", self.hours, self.minutes)
        } else if self.minutes > 0 {
            format!("{}m", self.minutes)
        } else {
            format!("{}s", self.seconds)
        }
    }

    /// Format remaining time as MM:SS
    pub fn format_countdown(remaining_seconds: u64) -> String {
        let hours = remaining_seconds / 3600;
        let minutes = (remaining_seconds % 3600) / 60;
        let seconds = remaining_seconds % 60;

        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{}:{:02}", minutes, seconds)
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurationError {
    InvalidFormat(String),
    InvalidMinutes(u32),
    InvalidSeconds(u32),
    ExceedsMaximum(u32),
    TooShort,
}

impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationError::InvalidFormat(input) => {
                write!(
                    f,
                    "Invalid format: '{}'. Use formats like '2h 30m', '150m', '2:30', or '2.5h'",
                    input
                )
            }
            DurationError::InvalidMinutes(m) => {
                write!(f, "Invalid minutes: {}. Minutes must be 0-59", m)
            }
            DurationError::InvalidSeconds(s) => {
                write!(f, "Invalid seconds: {}. Seconds must be 0-59", s)
            }
            DurationError::ExceedsMaximum(max) => {
                write!(f, "Duration exceeds maximum of {} hours", max)
            }
            DurationError::TooShort => {
                write!(f, "Duration must be at least 1 minute")
            }
        }
    }
}

impl std::error::Error for DurationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_creation() {
        let d = Duration::new(2, 30, 0).unwrap();
        assert_eq!(d.total_seconds, 9000);

        // Test validation
        assert!(Duration::new(9, 0, 0).is_err()); // Exceeds 8 hours
        assert!(Duration::new(0, 60, 0).is_err()); // Invalid minutes
        assert!(Duration::new(0, 0, 30).is_err()); // Too short
    }

    #[test]
    fn test_duration_parsing() {
        assert_eq!(Duration::parse("2h 30m").unwrap().total_seconds, 9000);
        assert_eq!(Duration::parse("150m").unwrap().total_seconds, 9000);
        assert_eq!(Duration::parse("2:30").unwrap().total_seconds, 9000);
        // Note: 2.5h should be 2 hours + 30 minutes = 9000 seconds, not 7200
        assert_eq!(Duration::parse("2.5h").unwrap().total_seconds, 9000);

        // Test error cases
        assert!(Duration::parse("abc").is_err());
        assert!(Duration::parse("10h").is_err());
    }

    #[test]
    fn test_duration_formatting() {
        let d = Duration::new(2, 30, 15).unwrap();
        assert_eq!(d.format(), "2h 30m");
        assert_eq!(Duration::format_countdown(9015), "2:30:15");
        assert_eq!(Duration::format_countdown(90), "1:30");
    }
}
