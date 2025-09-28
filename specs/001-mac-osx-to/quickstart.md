# Quickstart Guide: macOS Sleep Prevention Utility

## User Acceptance Testing Scenarios

This guide provides step-by-step scenarios to validate that the app meets all functional requirements from the specification.

## Prerequisites
- macOS 10.15+ (Catalina or newer)
- App built and installed from source or distributed package
- System permissions granted if prompted

---

## Test Scenario 1: Basic Sleep Prevention (FR-001 to FR-004)

### Objective
Verify the app runs as a system tray application and can prevent sleep for a specified duration.

### Steps
1. **Launch the application**
   - Double-click the app or run from Applications folder
   - **Expected**: App icon appears in system tray (menu bar)
   - **Expected**: No main window opens (tray-only app)

2. **Start sleep prevention**
   - Click the tray icon
   - **Expected**: Context menu appears with "Start Prevention" option
   - Click "Start Prevention"
   - **Expected**: Duration input dialog appears

3. **Set duration to 5 minutes**
   - Enter "5m" or "0:05" in the input field
   - Click "Start" or press Enter
   - **Expected**: Menu closes, tray icon changes to active state (sun icon)
   - **Expected**: Tooltip shows "5:00 remaining" when hovering

4. **Verify sleep prevention is active**
   - Wait 2-3 minutes without touching mouse/keyboard
   - **Expected**: Screen remains on (no display sleep)
   - **Expected**: Computer remains awake (no system sleep)
   - **Expected**: Tray tooltip updates countdown (e.g., "2:30 remaining")

5. **Verify automatic restoration**
   - Wait for full 5 minutes to elapse
   - **Expected**: Tray icon changes to expired state (checkmark icon)
   - **Expected**: Normal sleep behavior resumes
   - **Expected**: System can sleep again after inactivity

### Success Criteria
- ✅ App appears only in system tray
- ✅ Sleep prevention works for specified duration
- ✅ Visual feedback shows status changes
- ✅ Sleep behavior automatically restores

---

## Test Scenario 2: Manual Cancellation (FR-005)

### Objective
Verify users can manually stop sleep prevention before duration expires.

### Steps
1. **Start a longer prevention session**
   - Click tray icon → "Start Prevention"
   - Enter "30m" (30 minutes)
   - Click "Start"
   - **Expected**: Active prevention begins

2. **Cancel after 2 minutes**
   - Click tray icon
   - **Expected**: Menu shows "Stop Prevention" option (enabled)
   - Click "Stop Prevention"
   - **Expected**: Confirmation dialog appears

3. **Confirm cancellation**
   - Click "Yes" or "Stop" in confirmation dialog
   - **Expected**: Tray icon immediately changes to inactive state
   - **Expected**: Sleep behavior immediately resumes
   - **Expected**: No 30-minute timer continues running

### Success Criteria
- ✅ Can cancel active prevention
- ✅ Immediate sleep behavior restoration
- ✅ Clear confirmation process

---

## Test Scenario 3: Visual Status Indicators (FR-006, FR-007)

### Objective  
Verify tray icon correctly displays different states with appropriate visual feedback.

### Steps
1. **Test inactive state**
   - Ensure no prevention is running
   - **Expected**: Tray icon shows hollow moon (gray)
   - **Expected**: Tooltip shows "Sleep Prevention: Inactive"

2. **Test active state**
   - Start prevention for "10m"
   - **Expected**: Tray icon changes to filled sun (orange/yellow)
   - **Expected**: Tooltip shows remaining time (e.g., "9:45 remaining")

3. **Test countdown updates**
   - Wait and observe tooltip over 2-3 minutes
   - **Expected**: Countdown decreases every second
   - **Expected**: Format shows MM:SS (e.g., "7:42 remaining")

4. **Test expired state**
   - Let prevention session complete naturally
   - **Expected**: Tray icon changes to checkmark (green)
   - **Expected**: Tooltip shows "Prevention completed"
   - **Expected**: After ~5 seconds, returns to inactive state

### Success Criteria
- ✅ Three distinct icon states
- ✅ Real-time countdown display
- ✅ Clear visual transitions

---

## Test Scenario 4: Input Validation (FR-009, FR-014)

### Objective
Verify the app properly validates duration input and enforces the 8-hour maximum.

### Steps
1. **Test valid formats**
   - Try each format: "2h 30m", "150m", "2:30", "2.5h"
   - **Expected**: All formats accepted and parsed correctly
   - **Expected**: All result in same 2.5-hour duration

2. **Test invalid formats**
   - Try: "abc", "25:90", "-30m", "2.5.5h"
   - **Expected**: Clear error messages for each
   - **Expected**: Suggestions for valid formats shown

3. **Test maximum limit enforcement**
   - Try: "10h", "500m", "9:00"  
   - **Expected**: Error: "Maximum duration is 8 hours"
   - **Expected**: Suggestion to use shorter duration

4. **Test minimum limit**
   - Try: "0m", "0:00", ""
   - **Expected**: Error: "Minimum duration is 1 minute"

### Success Criteria
- ✅ Multiple input formats supported
- ✅ Clear validation error messages
- ✅ 8-hour maximum enforced
- ✅ Helpful format suggestions

---

## Test Scenario 5: System Integration (FR-008, FR-010)

### Objective
Verify proper handling of macOS system events during active prevention.

### Steps
1. **Test lid close handling** (on MacBook)
   - Start sleep prevention for "20m"
   - Close laptop lid for 30 seconds
   - Open lid
   - **Expected**: Screen turns back on immediately
   - **Expected**: Prevention session continues normally
   - **Expected**: No sleep occurred during lid close

2. **Test manual sleep attempt**
   - Start sleep prevention for "15m"
   - Apple Menu → Sleep (or ⌘+⌥+Power)
   - **Expected**: Sleep command is blocked
   - **Expected**: System remains awake
   - **Expected**: Prevention continues normally

3. **Test screensaver/display settings**
   - Set display sleep to 1 minute in System Preferences
   - Start sleep prevention for "10m"
   - Wait 2 minutes without input
   - **Expected**: Display remains on (no screensaver)
   - **Expected**: Prevention overrides display sleep setting

### Success Criteria
- ✅ Prevents both system and display sleep
- ✅ Handles lid close events correctly
- ✅ Overrides manual sleep commands
- ✅ Works with existing power settings

---

## Test Scenario 6: Multiple Instance Prevention (FR-015)

### Objective
Verify the app prevents multiple instances and handles duplicate launches gracefully.

### Steps
1. **Launch first instance**
   - Start the app normally
   - **Expected**: Tray icon appears

2. **Attempt second launch**
   - Try to launch the app again (double-click, command line, etc.)
   - **Expected**: No second tray icon appears
   - **Expected**: Existing tray icon briefly highlights or bounces
   - **Expected**: Focus returns to existing instance

3. **Verify single instance behavior**
   - Start prevention in the running instance
   - Try launching app again during active prevention
   - **Expected**: Active session continues uninterrupted
   - **Expected**: No conflicts or duplicate sessions

### Success Criteria
- ✅ Only one instance runs at a time
- ✅ Subsequent launches focus existing instance
- ✅ No session conflicts or duplicates

---

## Test Scenario 7: User Session Changes (FR-016)

### Objective
Verify proper behavior when user logs out or switches users during active prevention.

### Steps
1. **Test user logout**
   - Start sleep prevention for "60m"
   - Apple Menu → Log Out
   - **Expected**: Prevention session ends
   - **Expected**: Normal sleep behavior resumes
   - **Expected**: App cleanly shuts down

2. **Test fast user switching** (if enabled)
   - Start prevention for "30m"  
   - Switch to another user account
   - **Expected**: Prevention cancels in background
   - **Expected**: System can sleep normally for other user
   - Switch back to original user
   - **Expected**: App restarts with inactive state

### Success Criteria
- ✅ Prevention cancels on logout
- ✅ Clean shutdown during session changes
- ✅ No lingering sleep prevention

---

## Performance Validation

### Resource Usage Monitoring
1. **Memory usage**
   - Activity Monitor → Memory tab
   - **Expected**: App uses <50MB RAM during active prevention
   - **Expected**: No memory leaks over extended sessions

2. **CPU usage**
   - Activity Monitor → CPU tab
   - **Expected**: <1% CPU usage during countdown
   - **Expected**: Minimal system impact

3. **Battery impact** (on MacBook)
   - Start 4-hour prevention session on battery
   - Monitor battery percentage decline
   - **Expected**: Reasonable battery usage for extended display-on time
   - **Expected**: App provides battery warnings if needed

---

## Error Recovery Testing

### System Stress Scenarios
1. **Low battery handling**
   - Start prevention on <10% battery
   - **Expected**: Warning about battery drain
   - **Expected**: Option to continue or cancel

2. **System sleep override**
   - Start prevention during critical battery (<5%)
   - **Expected**: macOS may override prevention
   - **Expected**: App detects override and updates status

3. **Permission changes**
   - Revoke accessibility permissions during active session
   - **Expected**: Graceful error handling
   - **Expected**: Clear user guidance for restoration

---

## Completion Checklist

After running all scenarios, verify:

- [ ] All 16 functional requirements (FR-001 through FR-016) tested
- [ ] System tray integration works properly
- [ ] Sleep prevention is effective for both system and display
- [ ] Visual feedback is clear and helpful
- [ ] Input validation prevents errors
- [ ] Resource usage is reasonable
- [ ] Error handling is user-friendly
- [ ] App behaves well during system events

**Success**: All scenarios pass → App ready for production use
**Failure**: Any scenario fails → Return to development/debugging