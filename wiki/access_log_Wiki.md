# SigmaOS-access-log Component Wiki

## Overview
SigmaOS-access-log provides Linux Mint/Omarchy/Arch-inspired access logging component for the SigmaOS operating system. It demonstrates the logging subsystem and event tracking within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic log initialization.

## Principles
- Single Responsibility
- Logging Abstraction
- Fail-Fast on Logging Initialization Errors
- Deterministic Log Setup
- Clear Abstraction Over Logging APIs
- Composition over Inheritance for log extensions

## Architecture
SigmaOS-access-log follows a logging pattern:
- **Log Core**: Provides log event structures, severity levels, and output formatting
- **Event Tracking**: Manages event capture, rotation, and retention policies
- **Output Formatting**: Supports structured logging (JSON, plain text) and integration with external systems
- **Error Handling**: Fail-fast on invalid log configurations or output failures

## API
The component provides the following public APIs:
- `log::log(level, message)` - Log a message at the given severity level
- `log::get_level()` - Get the current logging level
- `log::set_level(level)` - Set the logging level
- `log::flush()` - Flush log output to storage
- Error types for logging initialization failures or output errors

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific logging configuration
2. **Act**: Execute the code under test (log event, level management)
3. **Assert**: Verify the expected logging results

Example:
```rust
#[test]
fn test_log_event_aaa() {
    // Arrange
    let level = "info";
    let message = "test log event";
    
    // Act
    let result = log::log(level, message);
    
    // Assert
    assert!(result.is_ok());
    assert!(log::get_level() == level);
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Logging initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all logging failure scenarios
- Unit tests provided for log event creation and level management
- Integration tests for full logging subsystem with output destinations

## TODO
- Add support for additional log levels and formatting options
- Expand output destination capabilities (file, stderr, external systems)
- Add log rotation and retention policy features
- Create integration tests with actual logging output destinations
- Implement log correlation and context tracking