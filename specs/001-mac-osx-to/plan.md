
# Implementation Plan: macOS Sleep Prevention Utility

**Branch**: `001-mac-osx-to` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-mac-osx-to/spec.md`

## Summary
Primary requirement: Create a macOS system tray application that prevents both display and system sleep for user-specified durations (up to 8 hours) with visual status feedback and intuitive controls. Technical approach: Tauri v2 leveraging JavaScript/TypeScript frontend with Rust backend for native macOS system integration, chosen based on user's JS/TS experience and cross-platform nature.

## Technical Context
**Language/Version**: JavaScript/TypeScript + Rust 1.75+ (Tauri v2 framework)
**Primary Dependencies**: Tauri v2, React/Vue.js (frontend), macOS IOKit (system sleep APIs)
**Storage**: Local preferences file (JSON/TOML) for app settings
**Testing**: Manual testing using quickstart.md scenarios (no automated test suite)
**Target Platform**: macOS 10.15+ (Catalina and newer)
**Project Type**: single (desktop app with system tray)
**Performance Goals**: <50ms tray menu response, <100MB memory usage, minimal CPU impact
**Constraints**: Battery efficiency priority, 8-hour maximum duration, single instance only
**Scale/Scope**: Single-user desktop app, ~5-10 tray menu items, 3 icon states

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Tidy First Philosophy
- [x] Changes follow small, safe, reversible steps (Tauri project structure enables incremental development)
- [x] Design eliminates complexity rather than managing it (Single responsibility: sleep prevention only)
- [x] Code communicates intent clearly to future developers (JS/TS frontend, well-documented Rust backend)

### Modern Toolchain Standards  
- [x] Development environment uses required tool versions:
  - Claude Code AI assistant (@anthropic-ai/claude-code@1.0.127+) ✓
  - GitHub Copilot CLI (@github/copilot@0.0.328+) ✓
  - npm package manager (npm@11.6.1+) ✓

### Code Quality Fundamentals
- [x] Design optimizes for readability over cleverness (Tauri's convention over configuration)
- [x] Self-documenting code with meaningful names (TypeScript interfaces for clear contracts)
- [x] Test strategy focuses on confidence, not coverage (Integration tests for tray behavior)
- [x] Guard clauses and helper variables planned for clarity (Time parsing, error handling)

### Change-Friendly Design
- [x] Tidying commits separated from behavior changes (Tauri enables clean separation)
- [x] Built for next developer's understanding (Standard Tauri project structure)
- [x] Options created over rigid implementations (Configurable via Tauri config)
- [x] Progressive enhancement approach (Basic tray → time input → visual feedback)

### Testing Strategy
- [x] Integration tests prioritized over unit tests (System tray behavior testing)
- [x] Tests focus on behavior, not implementation (Sleep prevention effectiveness)
- [x] User-facing functionality emphasized (Tray interactions, visual feedback)
## Project Structure

### Documentation (this feature)
```
specs/001-mac-osx-to/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
src-tauri/
├── src/
│   ├── lib.rs           # Tauri app entry point
│   ├── sleep_manager.rs # macOS sleep prevention logic
│   ├── tray_manager.rs  # System tray management
│   └── time_parser.rs   # Duration parsing utilities
├── Cargo.toml
├── tauri.conf.json      # Tauri configuration
└── icons/               # Tray icon states (inactive/active/expired)

src/
├── main.ts              # Frontend entry point
├── components/
│   ├── TrayMenu.vue     # Main tray menu component
│   ├── TimeInput.vue    # Duration input form
│   └── StatusDisplay.vue # Current status indicator
├── stores/
│   └── sleepSession.ts  # Pinia store for session state
└── utils/
    └── timeFormat.ts    # Time formatting utilities

docs/
└── manual-testing.md    # Manual testing checklist (from quickstart.md)
```

## Phase 0: Outline & Research

### Research Tasks Identified:
1. **Tauri v2 vs SwiftUI comparison for system tray apps**
2. **macOS sleep prevention APIs (IOKit vs NSProcessInfo)**
3. **Tauri system tray capabilities and limitations**
4. **macOS app lifecycle and user session handling**
5. **Icon design patterns for system tray status indication**

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

### Design Documents Generated:
1. **data-model.md**: Core entities (SleepSession, Duration, TrayState) with TypeScript interfaces
2. **contracts/tauri-commands.ts**: Frontend-backend API contracts for all Tauri commands  
3. **quickstart.md**: Comprehensive user acceptance testing scenarios covering all 16 functional requirements

### Key Design Decisions:
- **Tauri Commands**: 8 primary commands for sleep prevention, tray management, and configuration
- **Event-Driven Architecture**: Real-time updates via Tauri events (time-remaining, session-ended, etc.)
- **State Management**: Minimal in-memory state with TOML configuration persistence
- **Error Handling**: Graceful degradation with clear user feedback

**Output**: Design documents provide complete technical specification for implementation

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load contract definitions from `/contracts/tauri-commands.ts`
- Generate Rust backend tasks for each Tauri command implementation
- Generate TypeScript frontend tasks for UI components and state management
- Create manual testing validation tasks based on quickstart.md scenarios
- Follow implementation-first approach: setup → backend → frontend → manual validation

**Ordering Strategy**:
- Setup: Tauri project initialization, dependencies, tooling
- Backend: Rust command implementations (sleep prevention, tray management)  
- Frontend: Vue.js components and Pinia store integration
- Integration: Manual testing using quickstart.md scenarios
- Polish: Icon assets, error handling, performance optimization

**Estimated Output**: 20-25 numbered, ordered tasks in tasks.md (reduced due to no automated testing)

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitutional violations identified - design follows all principles*

| Constitutional Principle | Compliance Status | Notes |
|--------------------------|-------------------|-------|
| Tidy First Philosophy | ✅ PASS | Tauri enables small, reversible changes |
| Modern Toolchain | ✅ PASS | Uses specified npm, Claude, Copilot versions |
| Code Quality | ✅ PASS | TypeScript contracts, meaningful names |
| Change-Friendly Design | ✅ PASS | Clear separation of concerns, progressive enhancement |
| Testing Strategy | ✅ PASS | Integration-focused with behavior testing |

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)  
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved (via /clarify)
- [x] Complexity deviations documented (none required)

**Artifacts Generated**:
- [x] research.md - Technology decisions and patterns
- [x] data-model.md - Core entities and relationships  
- [x] contracts/tauri-commands.ts - Frontend-backend API contracts
- [x] quickstart.md - User acceptance testing scenarios
- [x] .github/copilot-instructions.md - AI assistant context

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*
- [ ] Changes follow small, safe, reversible steps
- [ ] Design eliminates complexity rather than managing it
- [ ] Code communicates intent clearly to future developers

### Modern Toolchain Standards  
- [ ] Development environment uses required tool versions:
  - Claude Code AI assistant (@anthropic-ai/claude-code@1.0.127+)
  - GitHub Copilot CLI (@github/copilot@0.0.328+)
  - npm package manager (npm@11.6.1+)

### Code Quality Fundamentals
- [ ] Design optimizes for readability over cleverness
- [ ] Self-documenting code with meaningful names
- [ ] Test strategy focuses on confidence, not coverage
- [ ] Guard clauses and helper variables planned for clarity

### Change-Friendly Design
- [ ] Tidying commits separated from behavior changes
- [ ] Built for next developer's understanding
- [ ] Options created over rigid implementations
- [ ] Progressive enhancement approach (simple first)

### Testing Strategy
- [ ] Integration tests prioritized over unit tests
- [ ] Tests focus on behavior, not implementation
- [ ] User-facing functionality emphasized
- [ ] Mocking at network boundaries only

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->
```
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh copilot`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [ ] Phase 0: Research complete (/plan command)
- [ ] Phase 1: Design complete (/plan command)
- [ ] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [ ] Initial Constitution Check: PASS
- [ ] Post-Design Constitution Check: PASS
- [ ] All NEEDS CLARIFICATION resolved
- [ ] Complexity deviations documented

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*
