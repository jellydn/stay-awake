# Data Model: macOS Sleep Prevention Utility

## Core Entities

### Sleep Session
```typescript
interface SleepSession {
  id: string;                    // Unique session identifier
  startTime: Date;              // When prevention started
  duration: Duration;           // Total prevention duration
  remainingTime: number;        // Seconds remaining
  status: SessionStatus;        // Current session state
  createdBy: string;           // User identifier (for multi-user support)
}
```

### Duration
```typescript
interface Duration {
  hours: number;               // 0-8 hours (enforced maximum)
  minutes: number;             // 0-59 minutes
  seconds: number;             // 0-59 seconds
  totalSeconds: number;        // Computed total for easier calculations
}
```

### Session Status
```typescript
enum SessionStatus {
  INACTIVE = 'inactive',       // No active prevention
  ACTIVE = 'active',          // Currently preventing sleep
  EXPIRED = 'expired',        // Duration completed, prevention ended
  CANCELLED = 'cancelled',    // User manually stopped
  ERROR = 'error'            // System error occurred
}
```

### Tray State
```typescript
interface TrayState {
  iconType: IconType;         // Current icon being displayed
  tooltipText: string;        // Hover text for tray icon
  menuItems: MenuItem[];      // Available menu options
  isVisible: boolean;         // Tray visibility state
}
```

### Icon Type
```typescript
enum IconType {
  INACTIVE = 'moon-hollow',    // Hollow moon (gray)
  ACTIVE = 'sun-filled',      // Filled sun (orange)  
  EXPIRED = 'check-mark'      // Checkmark (green)
}
```

### Menu Item
```typescript
interface MenuItem {
  id: string;                 // Unique menu item identifier
  label: string;              // Display text
  enabled: boolean;           // Whether item is clickable
  separator?: boolean;        // Is this a separator line
  action?: MenuAction;        // What happens when clicked
}
```

### Menu Action
```typescript
enum MenuAction {
  START_PREVENTION = 'start',
  STOP_PREVENTION = 'stop',
  SHOW_SETTINGS = 'settings',
  SHOW_ABOUT = 'about',
  QUIT_APP = 'quit'
}
```

### App Configuration
```typescript
interface AppConfig {
  maxDurationHours: number;    // Maximum allowed duration (8)
  defaultDuration: Duration;   // Default when app starts
  showNotifications: boolean;  // Enable system notifications
  autoStart: boolean;         // Start with system login
  theme: 'auto' | 'light' | 'dark'; // Icon theme preference
}
```

### System State
```typescript
interface SystemState {
  sleepAssertionId?: number;   // Active IOKit sleep assertion ID
  displayAssertionId?: number; // Active IOKit display assertion ID
  isPreventingSleep: boolean;  // Current prevention status
  lastError?: string;         // Most recent error message
  systemVersion: string;      // macOS version for compatibility
}
```

## Entity Relationships

### State Flow
```
INACTIVE → (user starts) → ACTIVE → (timer expires) → EXPIRED
    ↑                         ↓
    ← ← ← ← (user cancels) ← ←

ERROR ← (system failure) ← ACTIVE
    ↓
(auto-retry or manual reset) → INACTIVE
```

### Data Lifecycle

1. **Session Creation**: User inputs duration → Duration entity → SleepSession created
2. **Active Prevention**: SleepSession.status = ACTIVE → SystemState assertions created
3. **Tray Updates**: SessionStatus changes → TrayState.iconType updates → UI refresh
4. **Session End**: Timer expires → SystemState assertions released → status = EXPIRED

## Validation Rules

### Duration Constraints
- Hours: 0 ≤ hours ≤ 8
- Minutes: 0 ≤ minutes ≤ 59  
- Seconds: 0 ≤ seconds ≤ 59
- Total: 1 ≤ totalSeconds ≤ 28800 (8 hours)

### Session Rules
- Only one active session allowed at a time
- Session ID must be unique and non-empty
- StartTime cannot be in the future
- RemainingTime must be ≥ 0

### System State Rules
- AssertionIds must be valid when status is ACTIVE
- IsPreventingSleep must match assertion existence
- SystemVersion format: "XX.YY" or "XX.YY.ZZ"

## Storage Strategy

### In-Memory (Runtime)
- Current SleepSession (if active)
- Current TrayState
- Current SystemState
- Timer references and callbacks

### Persistent Storage (Configuration File)
- AppConfig settings
- User preferences
- No session history (privacy by design)

### File Location
```
~/Library/Preferences/com.macos-sleep-prevention.toml
```

### Configuration Format (TOML)
```toml
[app]
max_duration_hours = 8
show_notifications = true
auto_start = false
theme = "auto"

[default_duration]
hours = 2
minutes = 0
seconds = 0
```

## Error Handling

### Error States
- **PERMISSION_DENIED**: Missing system permissions
- **ASSERTION_FAILED**: IOKit API call failed
- **INVALID_DURATION**: User input validation failed
- **SYSTEM_OVERRIDE**: macOS force-overrode prevention (low battery)

### Error Recovery
- Automatic retry for transient failures
- Clear error messages for user-actionable issues
- Graceful degradation when APIs unavailable
- Reset to INACTIVE state on unrecoverable errors