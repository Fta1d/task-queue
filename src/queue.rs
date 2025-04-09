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

    pub fn peek<F, R>(&self, f: F) -> Result<Option<&T>, QueueError>
    where
        F: FnOnce(&T) -> R
    {
        let mut queue = self.lock_queue()?;
        Ok(queue.front().map(f))
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
