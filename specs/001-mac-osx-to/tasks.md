# Tasks: macOS Sleep Prevention Utility

**Input**: Design documents from `/specs/001-mac-osx-to/`
**Prerequisites**: plan.md (✓), research.md (✓), data-model.md (✓), contracts/ (✓)

## Path Conventions
Using Tauri v2 single project structure:
- **Backend**: `src-tauri/src/` (Rust)
- **Frontend**: `src/` (TypeScript + Vue.js)
- **Config**: Repository root
- **Icons**: `src-tauri/icons/`

## Phase 3.1: Setup & Project Initialization

- [x] T001 Create Tauri v2 project structure using `npm create tauri-app`
- [x] T002 Configure Cargo.toml with required dependencies (serde, tauri, tokio, cocoa)
- [x] T003 [P] Configure package.json with Vue.js and TypeScript dependencies
- [x] T004 [P] Setup tauri.conf.json for system tray application (no window, tray only)
- [x] T005 [P] Configure development environment (Vite, TypeScript, ESLint)
- [x] T006 Create tray icon assets (inactive/active/expired states) in src-tauri/icons/

## Phase 3.2: Backend Core Implementation

- [x] T007 [P] Implement Duration struct and parsing logic in src-tauli/src/time_parser.rs
- [x] T008 [P] Implement SleepSession entity and state management in src-tauri/src/session.rs
- [x] T009 Implement macOS sleep prevention using IOKit in src-tauri/src/sleep_manager.rs
- [ ] T010 Implement system tray management in src-tauri/src/tray_manager.rs
- [ ] T011 Create Tauri command: start_sleep_prevention in src-tauri/src/commands/prevention.rs
- [ ] T012 Create Tauri command: stop_sleep_prevention in src-tauri/src/commands/prevention.rs
- [ ] T013 Create Tauri command: get_sleep_status in src-tauri/src/commands/prevention.rs
- [ ] T014 [P] Create Tauri command: update_tray_icon in src-tauri/src/commands/tray.rs
- [ ] T015 [P] Create Tauri command: update_tray_menu in src-tauri/src/commands/tray.rs

## Phase 3.3: Configuration & System Integration

- [ ] T016 [P] Implement app configuration management in src-tauri/src/config.rs
- [ ] T017 [P] Create Tauri commands: get_app_config, update_app_config in src-tauri/src/commands/config.rs
- [ ] T018 [P] Create Tauri command: get_system_info in src-tauri/src/commands/system.rs
- [ ] T019 [P] Create Tauri command: validate_duration in src-tauri/src/commands/validation.rs
- [ ] T020 Implement user session change detection (logout/user switching) in src-tauri/src/session_monitor.rs
- [ ] T021 Implement single instance prevention in src-tauri/src/lib.rs
- [ ] T022 Setup Tauri app builder with all commands and system tray in src-tauri/src/lib.rs

## Phase 3.4: Frontend Implementation

- [ ] T023 [P] Create TypeScript interfaces (copy from contracts/tauri-commands.ts) in src/types/
- [ ] T024 [P] Setup Pinia store for sleep session state in src/stores/sleepSession.ts
- [ ] T025 [P] Create time formatting utilities in src/utils/timeFormat.ts
- [ ] T026 Create main tray menu component in src/components/TrayMenu.vue
- [ ] T027 Create duration input dialog component in src/components/TimeInput.vue
- [ ] T028 [P] Create status display component in src/components/StatusDisplay.vue
- [ ] T029 Implement Tauri command invocation service in src/services/tauriService.ts
- [ ] T030 Setup Vue.js app entry point and component mounting in src/main.ts

## Phase 3.5: Integration & Event Handling

- [ ] T031 Connect frontend store to backend events (session-started, session-ended, time-remaining)
- [ ] T032 Implement tray icon state updates based on session status
- [ ] T033 Implement tray menu dynamic updates (start/stop options based on state)
- [ ] T034 Add countdown timer display in tray tooltip
- [ ] T035 Implement error handling and user feedback for failed operations
- [ ] T036 Add battery level monitoring and warnings for extended sessions

## Phase 3.6: Polish & Finalization

- [ ] T037 [P] Add input validation with helpful error messages for duration formats
- [ ] T038 [P] Implement graceful shutdown and cleanup on app quit
- [ ] T039 [P] Add system notifications for session start/end (optional feature)
- [ ] T040 [P] Optimize memory usage and CPU impact during countdown
- [ ] T041 Create application bundle and test installation process
- [ ] T042 Run complete manual testing using quickstart.md scenarios

## Dependencies

**Sequential Dependencies**:
- T001 blocks T002-T006 (project must exist first)
- T007-T008 block T009-T015 (core entities before commands)
- T009 blocks T011-T013 (sleep manager before prevention commands)
- T010 blocks T014-T015 (tray manager before tray commands)
- T022 blocks T031-T036 (app setup before integration)
- T023-T030 block T031-T036 (frontend components before integration)

**Parallel Groups**:
```
Group 1 (Setup): T003, T004, T005, T006
Group 2 (Core Backend): T007, T008
Group 3 (Commands): T014, T015, T016, T017, T018, T019
Group 4 (Frontend Core): T023, T024, T025, T028
Group 5 (Polish): T037, T038, T039, T040
```

## Manual Testing Strategy

After T042, validate using scenarios from quickstart.md:
1. Basic sleep prevention (5-minute test)
2. Manual cancellation
3. Visual status indicators
4. Input validation (various formats)
5. System integration (lid close, manual sleep)
6. Multiple instance prevention
7. User session changes
8. Performance monitoring

## Estimated Timeline

**Total Tasks**: 42
**Estimated Duration**: 5-7 days (single developer, part-time)
- Setup: 1 day (T001-T006)
- Backend: 2 days (T007-T022)
- Frontend: 1.5 days (T023-T030)
- Integration: 1 day (T031-T036)
- Polish: 0.5 day (T037-T042)

## Notes

- **No automated tests**: Following user preference for manual testing only
- **MVP focus**: Core functionality first, optional features in polish phase
- **Tauri patterns**: Following Tauri v2 best practices for system tray apps
- **macOS specific**: IOKit APIs require macOS 10.15+ target
- **Performance targets**: <50ms tray response, <100MB memory usage