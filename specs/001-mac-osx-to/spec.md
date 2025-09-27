# Feature Specification: macOS Sleep Prevention Utility

**Feature Branch**: `001-mac-osx-to`  
**Created**: 2025-01-27  
**Status**: Draft  
**Input**: User description: "mac osx to keep it do not go to sleep in given time, e.g 2h 50 mins"

## User Scenarios & Testing

### Primary User Story
As a macOS user, I want to prevent my computer from going to sleep for a specific duration (like 2 hours and 50 minutes) through a convenient system tray application so that I can ensure long-running tasks complete without interruption or maintain screen visibility during presentations/monitoring while having easy access to controls and status.

### Acceptance Scenarios
1. **Given** I have a macOS computer that normally sleeps after inactivity, **When** I launch the system tray app and set duration to "2h 50m", **Then** the computer stays awake for exactly 2 hours and 50 minutes and the tray icon shows active status
2. **Given** the utility is preventing sleep, **When** I click the tray icon and select cancel, **Then** the computer immediately resumes normal sleep behavior and tray icon returns to inactive state
3. **Given** I click the system tray icon, **When** I enter an invalid time format, **Then** the system shows a clear error message with valid format examples
4. **Given** the utility is running, **When** I close the laptop lid or trigger other sleep events, **Then** the system prevents sleep and the tray shows remaining time
5. **Given** the app is active, **When** I hover over or click the tray icon, **Then** I can see the remaining time and have access to cancel/modify options

### Edge Cases
- What happens when the system is forced to restart during the prevention period?
- How does the system handle extremely long durations (e.g., over 8 hours maximum limit)?
- What occurs if the user tries to launch the app when it's already running (app prevents multiple instances by focusing existing tray)?
- How does the utility interact with existing power management settings?
- What happens if the user logs out or switches users while the tray app is active (app cancels prevention and restores normal sleep)?
- How should the tray icon behave when the system tray is hidden or full?

## Requirements

### Functional Requirements
- **FR-001**: System MUST run as a macOS system tray (menu bar) application
- **FR-002**: System MUST accept time duration input in human-readable formats through tray interface (e.g., "2h 50m", "150m", "2:50", "2.5h")
- **FR-003**: System MUST prevent macOS from entering sleep mode for the specified duration
- **FR-004**: System MUST automatically restore normal sleep behavior after the duration expires
- **FR-005**: Users MUST be able to cancel the sleep prevention through the tray menu before the duration expires
- **FR-006**: System tray icon MUST display remaining time until sleep prevention ends
- **FR-007**: System tray icon MUST show clear visual status indication using different icon shapes for inactive, active, and expired states
- **FR-008**: System MUST prevent both display sleep and system sleep during the specified period (screen stays on and computer awake)
- **FR-009**: System MUST validate time input and provide helpful error messages for invalid formats through tray interface
- **FR-010**: System MUST handle system events (lid close, manual sleep attempts) according to prevention rules
- **FR-011**: System MUST provide tray menu options for starting, stopping, and configuring sleep prevention
- **FR-012**: System MUST persist in system tray until manually quit by user
- **FR-013**: System tray MUST be accessible via click/right-click with intuitive menu options
- **FR-014**: System MUST enforce a maximum duration limit of 8 hours to prevent excessive battery drain and system stress
- **FR-015**: System MUST prevent multiple instances by bringing existing tray app to focus and ignoring new launch attempts
- **FR-016**: System MUST cancel active sleep prevention and restore normal sleep behavior when user logs out or switches users

### Key Entities
- **Sleep Session**: Represents an active sleep prevention period with start time, duration, and remaining time
- **Time Duration**: Represents user-specified time periods with hours, minutes, and total seconds
- **System State**: Current sleep prevention status (active, inactive, expired, cancelled)
- **Tray Interface**: System tray icon and menu system with status display and user controls
- **Menu Actions**: Available user actions through tray menu (start, stop, configure, quit)

## Clarifications

### Session 2025-01-27
- Q: What level of sleep prevention should the app provide? → A: Prevent both system and display sleep (screen stays on and computer awake)
- Q: What should be the maximum allowed sleep prevention duration? → A: 8 hours maximum
- Q: How should the app handle multiple launch attempts? → A: Prevent multiple instances (show existing tray app, ignore new launches)
- Q: How should the tray icon indicate sleep prevention status? → A: Icon shape change (different symbols for each state: inactive, active, expired)
- Q: How should the app handle user logout or user switching during active sleep prevention? → A: Cancel prevention (restore normal sleep when user logs out or switches)

## Review & Acceptance Checklist

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous  
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed
