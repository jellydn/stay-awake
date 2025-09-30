use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use serde::{Deserialize, Serialize};

// IOKit power management constants
const K_IO_PM_ASSERTION_TYPE_NO_IDLE_SLEEP: &str = "NoIdleSleepAssertion";
const K_IO_PM_ASSERTION_TYPE_NO_DISPLAY_SLEEP: &str = "NoDisplaySleepAssertion";
const K_IO_PM_ASSERTION_LEVEL_ON: u32 = 255;

// IOKit assertion ID type
type IOPMAssertionID = u32;

// External C functions from IOKit
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: u32,
        assertion_name: CFStringRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> i32;

    fn IOPMAssertionRelease(assertion_id: IOPMAssertionID) -> i32;
}

/// Sleep prevention manager for macOS
#[derive(Debug)]
pub struct SleepManager {
    system_assertion_id: Option<IOPMAssertionID>,
    display_assertion_id: Option<IOPMAssertionID>,
    is_preventing_sleep: bool,
}

impl SleepManager {
    /// Create a new sleep manager
    pub fn new() -> Self {
        Self {
            system_assertion_id: None,
            display_assertion_id: None,
            is_preventing_sleep: false,
        }
    }

    /// Start preventing both system and display sleep
    pub fn start_prevention(&mut self, reason: &str) -> Result<(), SleepError> {
        if self.is_preventing_sleep {
            return Err(SleepError::AlreadyPreventing);
        }

        // Create system sleep assertion
        let system_id = self.create_assertion(
            K_IO_PM_ASSERTION_TYPE_NO_IDLE_SLEEP,
            &format!("Sleep Prevention: {}", reason),
        )?;

        // Create display sleep assertion
        let display_id = self.create_assertion(
            K_IO_PM_ASSERTION_TYPE_NO_DISPLAY_SLEEP,
            &format!("Display Prevention: {}", reason),
        )?;

        self.system_assertion_id = Some(system_id);
        self.display_assertion_id = Some(display_id);
        self.is_preventing_sleep = true;

        log::info!(
            "Started sleep prevention - System ID: {}, Display ID: {}",
            system_id,
            display_id
        );
        Ok(())
    }

    /// Stop sleep prevention
    pub fn stop_prevention(&mut self) -> Result<(), SleepError> {
        if !self.is_preventing_sleep {
            return Err(SleepError::NotPreventing);
        }

        let mut errors = Vec::new();

        // Release system assertion
        if let Some(id) = self.system_assertion_id.take() {
            if let Err(e) = self.release_assertion(id) {
                errors.push(format!("System assertion release failed: {}", e));
            }
        }

        // Release display assertion
        if let Some(id) = self.display_assertion_id.take() {
            if let Err(e) = self.release_assertion(id) {
                errors.push(format!("Display assertion release failed: {}", e));
            }
        }

        self.is_preventing_sleep = false;

        if errors.is_empty() {
            log::info!("Sleep prevention stopped successfully");
            Ok(())
        } else {
            let error_msg = errors.join("; ");
            log::error!("Sleep prevention stopped with errors: {}", error_msg);
            Err(SleepError::ReleaseError(error_msg))
        }
    }

    /// Check if currently preventing sleep
    pub fn is_preventing_sleep(&self) -> bool {
        self.is_preventing_sleep
    }

    /// Get current assertion IDs for debugging
    pub fn get_assertion_ids(&self) -> (Option<IOPMAssertionID>, Option<IOPMAssertionID>) {
        (self.system_assertion_id, self.display_assertion_id)
    }

    /// Create a power management assertion
    fn create_assertion(
        &self,
        assertion_type: &str,
        name: &str,
    ) -> Result<IOPMAssertionID, SleepError> {
        unsafe {
            // Create CFString for assertion type
            let cf_type = CFString::new(assertion_type);

            // Create CFString for assertion name
            let cf_name = CFString::new(name);

            let mut assertion_id: IOPMAssertionID = 0;

            // Create the assertion
            let result = IOPMAssertionCreateWithName(
                cf_type.as_CFTypeRef() as CFStringRef,
                K_IO_PM_ASSERTION_LEVEL_ON,
                cf_name.as_CFTypeRef() as CFStringRef,
                &mut assertion_id,
            );

            if result == 0 {
                // kIOReturnSuccess
                Ok(assertion_id)
            } else {
                Err(SleepError::AssertionCreateFailed(result))
            }
        }
    }

    /// Release a power management assertion
    fn release_assertion(&self, assertion_id: IOPMAssertionID) -> Result<(), SleepError> {
        unsafe {
            let result = IOPMAssertionRelease(assertion_id);

            if result == 0 {
                // kIOReturnSuccess
                Ok(())
            } else {
                Err(SleepError::AssertionReleaseFailed(assertion_id, result))
            }
        }
    }

    /// Force cleanup all assertions (emergency cleanup)
    pub fn force_cleanup(&mut self) {
        log::warn!("Force cleanup of sleep assertions");

        if let Some(id) = self.system_assertion_id.take() {
            let _ = self.release_assertion(id);
        }

        if let Some(id) = self.display_assertion_id.take() {
            let _ = self.release_assertion(id);
        }

        self.is_preventing_sleep = false;
    }

    /// Check system capabilities
    pub fn check_capabilities() -> SystemCapabilities {
        // For now, assume macOS 10.15+ supports all features
        // In production, you'd check actual system version
        SystemCapabilities {
            supports_io_kit: true,
            supports_display_sleep_prevention: true,
            supports_system_sleep_prevention: true,
            macos_version: Self::get_macos_version(),
        }
    }

    fn get_macos_version() -> String {
        // Simple version detection - in production use proper API
        "Unknown".to_string()
    }
}

impl Drop for SleepManager {
    /// Ensure cleanup on drop
    fn drop(&mut self) {
        if self.is_preventing_sleep {
            log::warn!("SleepManager dropped while still preventing sleep - cleaning up");
            self.force_cleanup();
        }
    }
}

impl Default for SleepManager {
    fn default() -> Self {
        Self::new()
    }
}

/// System capabilities for sleep prevention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    pub supports_io_kit: bool,
    pub supports_display_sleep_prevention: bool,
    pub supports_system_sleep_prevention: bool,
    pub macos_version: String,
}

/// Sleep management errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SleepError {
    AlreadyPreventing,
    NotPreventing,
    AssertionCreateFailed(i32),
    AssertionReleaseFailed(IOPMAssertionID, i32),
    ReleaseError(String),
    SystemNotSupported,
    PermissionDenied,
}

impl std::fmt::Display for SleepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SleepError::AlreadyPreventing => {
                write!(f, "Sleep prevention is already active")
            }
            SleepError::NotPreventing => {
                write!(f, "No active sleep prevention to stop")
            }
            SleepError::AssertionCreateFailed(code) => {
                write!(f, "Failed to create power assertion (error code: {})", code)
            }
            SleepError::AssertionReleaseFailed(id, code) => {
                write!(
                    f,
                    "Failed to release power assertion {} (error code: {})",
                    id, code
                )
            }
            SleepError::ReleaseError(msg) => {
                write!(f, "Error releasing assertions: {}", msg)
            }
            SleepError::SystemNotSupported => {
                write!(f, "Sleep prevention not supported on this system")
            }
            SleepError::PermissionDenied => {
                write!(
                    f,
                    "Permission denied - may require accessibility permissions"
                )
            }
        }
    }
}

impl std::error::Error for SleepError {}

/// High-level sleep prevention interface
pub struct SleepPrevention {
    manager: SleepManager,
}

impl SleepPrevention {
    pub fn new() -> Self {
        Self {
            manager: SleepManager::new(),
        }
    }

    /// Start preventing sleep with reason
    pub fn start(&mut self, reason: &str) -> Result<(), SleepError> {
        self.manager.start_prevention(reason)
    }

    /// Stop preventing sleep
    pub fn stop(&mut self) -> Result<(), SleepError> {
        self.manager.stop_prevention()
    }

    /// Check if active
    pub fn is_active(&self) -> bool {
        self.manager.is_preventing_sleep()
    }

    /// Get system info
    pub fn system_info(&self) -> SystemCapabilities {
        SleepManager::check_capabilities()
    }
}

impl Default for SleepPrevention {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_manager_creation() {
        let manager = SleepManager::new();
        assert!(!manager.is_preventing_sleep());
        assert_eq!(manager.get_assertion_ids(), (None, None));
    }

    #[test]
    fn test_system_capabilities() {
        let caps = SleepManager::check_capabilities();
        assert!(caps.supports_io_kit); // Should be true on macOS
    }

    #[test]
    fn test_sleep_prevention_interface() {
        let prevention = SleepPrevention::new();
        assert!(!prevention.is_active());

        let info = prevention.system_info();
        assert!(info.supports_io_kit);
    }

    // Note: Actual IOKit tests would require running on macOS
    // and might need elevated permissions
}
