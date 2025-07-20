# std_async Test Suite

This directory contains comprehensive tests for the std_async runtime implementation.

## Test Overview

### Unit Tests
Located in `src/` directories alongside the source code:

- **`runtime/executor_tests.rs`** - Tests for the async executor
  - Task spawning and completion
  - Polling behavior
  - Waker integration
  - Multiple concurrent futures

- **`runtime/sleep_tests.rs`** - Tests for the Sleep future
  - Duration handling
  - Polling states (Ready/Pending)
  - Timing accuracy

- **`runtime/waker_tests.rs`** - Tests for the custom waker implementation
  - Waker creation and cloning
  - Wake operations
  - Memory safety

- **`runtime/sender_tests.rs`** - Tests for TCP sending
  - Data transmission
  - Non-blocking behavior
  - Large data handling
  - Error conditions

- **`runtime/reciever_tests.rs`** - Tests for TCP receiving
  - Data reception
  - Chunked data handling
  - Connection lifecycle
  - Buffer management

- **`data/data_layer_tests.rs`** - Tests for serialization/deserialization
  - Round-trip serialization
  - Error handling (invalid UTF-8, truncated data)
  - Large data handling
  - Edge cases (empty strings, max values)

### Integration Tests
Located in `tests/integration_tests.rs`:

- **Complete async workflow** - End-to-end executor usage
- **TCP with serialization** - Full network stack testing
- **Concurrent operations** - Multiple simultaneous tasks
- **Complex async workflows** - Multi-phase async operations
- **Error handling** - Async error propagation

### Benchmark Tests
Located in `tests/benchmark_tests.rs`:

- **Performance testing** - Many concurrent tasks
- **Sleep precision** - Timing accuracy verification
- **Memory usage** - Resource management validation
- **Concurrent sleep operations** - Scalability testing

## Running Tests

### Quick Test Run
```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run only benchmarks
cargo test --test benchmark_tests
```

### Using the Test Script
```bash
./run_tests.sh
```

### Individual Test Categories
```bash
# Test specific modules
cargo test executor_tests
cargo test sleep_tests
cargo test data_layer_tests

# Test with output
cargo test -- --nocapture

# Test with timing info
cargo test -- --nocapture --test-threads=1
```

## Test Features

### What's Tested

1. **Core Runtime**
   - Task execution and lifecycle
   - Polling behavior
   - Waker functionality
   - Sleep timing accuracy

2. **Network Operations**
   - TCP sending and receiving
   - Non-blocking I/O
   - Large data transfers
   - Connection handling

3. **Data Layer**
   - Serialization correctness
   - Deserialization error handling
   - Unicode support
   - Protocol compliance

4. **Integration**
   - Complete client-server workflows
   - Concurrent operations
   - Performance characteristics
   - Memory management

### Error Scenarios Tested

- Invalid serialized data
- Network connection failures
- Truncated data streams
- Invalid UTF-8 sequences
- Task panics and recovery
- Resource exhaustion

### Performance Verification

- Task spawning overhead
- Polling efficiency
- Sleep timing precision
- Memory usage patterns
- Concurrent task scaling

## Test Requirements

- No external dependencies (only std library)
- Cross-platform compatibility
- Deterministic behavior where possible
- Comprehensive error coverage
- Performance regression detection

## Adding New Tests

When adding new functionality:

1. Add unit tests in the appropriate `*_tests.rs` file
2. Update integration tests if the change affects end-to-end behavior
3. Add benchmark tests for performance-critical features
4. Update this README if new test categories are added

## Debugging Tests

```bash
# Run tests with debug output
RUST_LOG=debug cargo test -- --nocapture

# Run a single test
cargo test test_executor_spawn_simple_future -- --nocapture

# Run tests with backtraces
RUST_BACKTRACE=1 cargo test
```
