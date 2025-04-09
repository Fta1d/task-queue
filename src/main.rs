mod error;
mod queue;

use crate::error::QueueError;

fn main() {
    let err = QueueError::Full;
    println!("{}", err);
}
