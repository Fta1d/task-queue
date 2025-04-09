/// standard libs
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum QueueError {
    Full,           // Queue has reached capacity
    Timeout,        // Operation timed out
    Shutdown,       // Queue has been shut down
    Disconnected,   // Queue has been disconnected
    Lock            // Failed to acquire lock
}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            QueueError::Full => write!(f, "Queue is full!"),
            QueueError::Disconnected => write!(f, "Queue has been disconnected!"),
            QueueError::Shutdown => write!(f, "Task has been shut down!"),
            QueueError::Timeout => write!(f, "Queue timed out!"),
            QueueError::Lock => write!(f, "Cannot lock thread!")
        }
    }
}

impl Error for QueueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // No need to unwrap error, because QueueError does not
        // have any source errors
        None
    }
}