# Research: macOS Sleep Prevention Utility

## Technology Stack Decision: Tauri v2 vs SwiftUI

### Decision: Tauri v2
**Rationale**: 
- Leverages existing JavaScript/TypeScript experience
- Cross-platform potential for future Windows/Linux versions  
- Modern web technologies for UI development
- Rust backend provides native macOS API access
- Active community and documentation

### Alternatives Considered:
- **SwiftUI**: More native but requires learning Swift from scratch
- **Electron**: Heavier resource usage, less native system integration
- **Native Rust**: No web UI, requires GUI framework learning

## macOS Sleep Prevention APIs

### Decision: IOKit Power Management
**Rationale**:
- Direct control over system and display sleep
- Fine-grained power assertions
- Reliable across macOS versions
- Well-documented C APIs with Rust bindings

### Implementation Pattern:
```rust
use core_foundation::base::TCFType;
use io_kit_sys::*;

// Prevent system sleep
let assertion_id = IOPMAssertionCreateWithName(
    kIOPMAssertionTypeNoIdleSleep,
    kIOPMAssertionLevelOn,
    reason_string,
    &mut assertion_id
);

// Prevent display sleep  
let display_assertion_id = IOPMAssertionCreateWithName(
    kIOPMAssertionTypeNoDisplaySleep,
    kIOPMAssertionLevelOn,
    reason_string,
    &mut display_assertion_id
);
```

### Alternatives Considered:
- **NSProcessInfo**: Higher-level but less control
- **caffeinate**: Command-line tool, harder to integrate
- **Third-party crates**: Less control, potential maintenance issues

## Tauri System Tray Integration

### Decision: Tauri Built-in System Tray
**Rationale**:
- Native integration with Tauri lifecycle
- Cross-platform tray menu support
- Event-driven architecture matches our needs
- Icon state management built-in

### Implementation Pattern:
```rust
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem};

let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("start", "Start Prevention"))
    .add_item(CustomMenuItem::new("stop", "Stop Prevention"))
    .add_separator()
    .add_item(CustomMenuItem::new("quit", "Quit"));

let system_tray = SystemTray::new().with_menu(tray_menu);
```

## User Session Lifecycle Handling

### Decision: macOS Notification Center Observer
**Rationale**:
- Reliable session change detection
- Standard macOS pattern
- Integrates well with Tauri event system

### Implementation Pattern:
```rust
use objc::*;
use cocoa::base::*;

// Listen for user session changes
NSNotificationCenter::defaultCenter().addObserver_selector_name_object(
    observer,
    sel!(sessionDidEnd:),
    NSString::alloc().init_str("NSSessionDidEndNotification"),
    ptr::null_mut()
);
```

## Icon Design Strategy

### Decision: Simple Geometric Shapes with System Colors
**Rationale**:
- Follows macOS Human Interface Guidelines
- Clear at small sizes (16x16 points)
- Accessible color contrast
- Easy to distinguish states

### Icon States:
- **Inactive**: Hollow moon symbol (system gray)
- **Active**: Filled sun symbol (system orange) 
- **Expired**: Checkmark symbol (system green)

### File Format: Template Images (.png)
- Supports macOS dark/light mode adaptation
- Tauri handles system tray icon rendering
- 16x16, 32x32, and 64x64 pixel versions

## Performance Considerations

### Timer Implementation: Tokio Async Runtime
**Rationale**:
- Non-blocking countdown updates
- Integrates with Tauri's async event system
- Minimal CPU usage for timer operations

### Memory Management: Minimal State Storage
**Decision**: Keep only active session data in memory
- No persistent history or logging
- Configuration in simple TOML file
- Target <50MB total memory usage

## Development Workflow

### Testing Strategy:
1. **Unit Tests**: Time parsing and validation logic
2. **Integration Tests**: System tray menu interactions  
3. **Manual Testing**: Actual sleep prevention verification
4. **Performance Testing**: Battery usage and memory profiling

### Build Process:
1. Tauri development server for rapid iteration
2. Automated builds for macOS universal binaries
3. Code signing for distribution (future consideration)

## Risk Mitigation

### Known Limitations:
- **System Updates**: May reset power management policies
- **Battery Critical**: macOS may override sleep prevention
- **Accessibility**: VoiceOver compatibility needs testing
- **Permissions**: May require accessibility permissions in future macOS versions

### Mitigation Strategies:
- Graceful degradation when APIs unavailable
- Clear user feedback when prevention fails
- Respect system critical battery states
- Comprehensive error handling and logging