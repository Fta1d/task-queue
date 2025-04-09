// standard libs
use std::collections::VecDeque;
use std::sync::{Mutex, Condvar, MutexGuard};

// own Error crate
use crate::error::QueueError;

const DEFAULT_CAPACITY: usize = 5;

pub struct TaskQueue<T> {
    queue: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    max_size: Option<usize>
}

impl<T> TaskQueue<T> {
    pub fn new(size: Option<usize>) -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(size.unwrap_or(DEFAULT_CAPACITY))),
            not_empty: Condvar::new(),
            max_size: size
        }
    }

    fn lock_queue(&self) -> Result<MutexGuard<'_, VecDeque<T>>, QueueError> {
        match self.queue.lock() {
            Ok(guard) => Ok(guard),
            Err(err) => {
                // Logging the error
                eprintln!("Lock error: {:?}", err);
                Err(QueueError::Lock)
            }
        }
    }

    pub fn push(&self, task: T) -> Result<(), QueueError> {
        let mut queue = self.lock_queue();

        // Check whether tasks do not overcome max_size
        if let Some(max_size) = self.max_size {
            if queue.len() >= max_size {
                return Err(QueueError::Full);
            }
        }

        queue.push_back(task);
        self.not_empty.notify_one();
        Ok(())
    }

    pub fn pop(&self) -> Result<Option<T>, QueueError> {
        let mut queue = self.lock_queue()?;

        // No need to check whether queue empty or not.
        // Pop_front method doing that for us
        Ok(queue.pop_front())
    }

    pub fn len(&self) -> Result<usize, QueueError> {
        let mut queue = self.lock_queue()?;

        Ok(queue.len())
    }

    /// The issue with peek is that it returns reference to data (&T),
    /// which isn't protected by mutex, but mutex itself unlocks at the
    /// end of a method (when MutexGuard goes out of sight). It creates
    /// dangerous reference, which points to the data that can be reached
    /// by any thread.
    ///
    /// # Arguments
    ///
    /// * `&self` - reference to the queue object
    /// * `f` - function that takes a reference to a queue element of type `T` and returns a result of type `R`
    ///
    /// # Returns
    ///
    /// * `Result<Option<&T>, QueueError>` - on success returns an `Option` with a reference to the element,
    ///   which can be `None` if the queue is empty, or a `QueueError` on failure
    ///
    /// # Errors
    ///
    /// Returns `QueueError` if the queue could not be locked for reading.
    ///
    /// # Note
    ///
    /// Note that although the function takes a parameter `f: F` which transforms `&T` into `R`,
    /// the return signature still contains `&T`, not `R`. This might be an inconsistency
    /// and requires verification.

    pub fn peek<F, R>(&self, f: F) -> Result<R, QueueError>
    where
        F: FnOnce(Option<&T>) -> R
    {
        // Lock the queue for safe access
        // The ? operator will return an error if lock_queue() fails
        let mut queue = self.lock_queue()?;

        // front() returns Option<&T> - a reference to the first element, if it exists
        // map(f) applies function f to the element, if present
        // Wrap the result in Ok to return Result
        Ok(f(queue.front()))
    }

    pub fn is_empty(&self) -> Result<bool, QueueError> {
        let mut queue = self.lock_queue()?;

        Ok(queue.is_empty())
    }

    pub fn clear(&self) -> Result<(), QueueError> {
        let mut queue = self.lock_queue()?;

        queue.clear();
        Ok(())
    }
}
