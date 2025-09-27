<!--
Version change: Initial → 1.0.0
Modified principles: All (initial creation)
Added sections: Core Principles, Development Environment, Code Quality Standards, Governance
Removed sections: None
Templates requiring updates: ⚠ pending (need to check template alignment)
Follow-up TODOs: None
-->

# macOS Do Not Sleep Constitution

## Core Principles

### I. Tidy First Philosophy
All code changes MUST follow Kent Beck's "Tidy First?" approach: make big changes through small, safe, reversible steps. Code is communication between humans, not just computers. Balance current effort against future options and eliminate problems rather than managing complexity.

**Rationale**: This ensures maintainable code that future developers can understand and modify safely.

### II. Modern Toolchain Standards
Development environment MUST maintain current versions of core tools:
- Claude Code AI assistant (@anthropic-ai/claude-code@1.0.127 or newer)
- GitHub Copilot CLI (@github/copilot@0.0.328 or newer)  
- npm package manager (npm@11.6.1 or newer)

**Rationale**: Modern AI-assisted development requires up-to-date tooling for optimal performance and security.

### III. Code Quality Fundamentals (NON-NEGOTIABLE)
- Don't solve problems, eliminate them - reduce complexity rather than manage it
- Optimize for readability over clever solutions
- Write self-documenting code with meaningful names
- Test for confidence, not coverage metrics
- Use guard clauses and extract helper variables for clarity

**Rationale**: Code quality directly impacts development velocity and reduces maintenance burden.

### IV. Change-Friendly Design
- Separate tidying commits from behavior changes
- Build for the next developer's understanding
- Create options over rigid implementations for uncertain requirements
- Use progressive enhancement - start simple, add complexity when needed

**Rationale**: Facilitates safe refactoring and reduces risk when making modifications.

### V. Testing Strategy
Follow Kent C. Dodds' Testing Trophy approach:
- Prioritize integration tests over isolated unit tests
- Test behavior, not implementation details
- Focus on user-facing functionality
- Mock at network boundaries, not internal functions

**Rationale**: Provides higher confidence in system behavior while maintaining test maintainability.

## Development Environment

All developers MUST maintain a standardized environment with specified tool versions. Package managers and AI assistants must be kept current to ensure consistent development experience and security compliance.

Global npm packages serve as the foundation for AI-assisted development workflow and must be updated regularly according to release schedules.

## Code Quality Standards

### Performance Practices
- Optimize for user-centric metrics (loading, interaction)
- Measure before optimizing - avoid premature micro-optimizations
- Use progressive enhancement patterns
- Implement lazy loading for non-critical resources

### Collaboration Standards
- Express intent clearly through naming and structure
- Document decisions, not implementation details
- Write clear commit messages separating tidying from features
- Focus code reviews on correctness, simplicity, and maintainability

## Governance

This constitution supersedes all other development practices. All pull requests and code reviews MUST verify compliance with these principles. 

Amendments require:
1. Documentation of proposed changes and rationale
2. Team approval through standard review process
3. Migration plan for existing code if applicable

Complexity MUST be justified against these principles. When in doubt, choose simplicity and eliminate rather than manage problems.

**Version**: 1.0.0 | **Ratified**: 2025-01-27 | **Last Amended**: 2025-01-27