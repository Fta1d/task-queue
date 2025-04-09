# TaskQueue Library

A thread-safe task queue implementation in Rust for managing asynchronous workloads.

## Overview

This library provides a robust, thread-safe task queue implementation designed for high-performance concurrent systems. It allows you to safely enqueue tasks from multiple producer threads and process them with one or more consumer threads.

## Components

### TaskQueue<T>

A generic, thread-safe queue that can store tasks of any type.

```rust
pub struct TaskQueue<T> {
    queue: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    max_size: Option<usize>
}
```

#### Features:

- Thread-safe operations via Mutex
- Optional capacity limit
- Blocking and non-blocking operations
- Condition variable for efficient waiting on empty queues

#### Methods:

- `new(size: Option<usize>)` - Create a new queue with optional size limit
- `push(task: T)` - Add a task to the queue
- `pop()` - Remove and return the next task
- `peek()` - Access the next task without removing it
- `len()` - Get the current queue length
- `is_empty()` - Check if the queue is empty
- `clear()` - Remove all tasks from the queue

### Error Handling

The library includes a comprehensive error type for handling queue-related issues:

```rust
pub enum QueueError {
    Full,          // Queue has reached capacity
    Timeout,       // Operation timed out
    Shutdown,      // Queue has been shut down
    Disconnected,  // Queue has been disconnected
    Lock           // Failed to acquire lock
}
```

## Usage Example

```rust
use task_queue::{TaskQueue, QueueError};

// Create a task queue with a maximum size of 10
let queue = TaskQueue::new(Some(10));

// Add tasks to the queue
queue.push(42)?;
queue.push(100)?;

// Process tasks
while let Some(task) = queue.pop()? {
    println!("Processing task: {}", task);
}
```

## Thread Safety

All operations on the `TaskQueue` are thread-safe, allowing multiple threads to push and pop tasks concurrently without race conditions.

## Error Handling

The library uses Rust's `Result` type with custom `QueueError` enum for comprehensive error handling. All operations that could fail return a `Result` type.

## Performance Considerations

- The queue uses a mutex for thread safety, which may cause contention under high load
- A condition variable is used to efficiently wait for new tasks
- Optional size limits can prevent memory exhaustion

## TODO
Implement the `Worker<T>` structure, which:
   * Launches a background thread for processing tasks from `TaskQueue<T>`
   * Accepts a task handler function as a parameter during creation
   * Provides methods for starting, stopping, and checking the work status
