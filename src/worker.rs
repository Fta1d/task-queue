use std::sync::Arc;
use crate::queue::TaskQueue;

pub struct Worker<T> {
    queue: Arc<TaskQueue<T>>,
    handler: Box<dyn Fn(T) + Send + 'static>
}

impl<T: Send + 'static> Worker<T> {
    pub fn new(queue: Arc<TaskQueue<T>>, handler: impl Fn(T) + Send + 'static) -> Self {
        Self {
            queue,
            handler: Box::new(handler)
        }
    }

    /// #TODO
    /// Methods for start, stop, state check...
    /// Add fields for struct to handle the thread
}