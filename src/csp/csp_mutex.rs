use std::sync::{Mutex as StdMutex, MutexGuard as StdMutexGuard};

pub trait CspMutex {
    type Guard;

    fn lock(&self) -> Self::Guard;
}

impl<T> CspMutex for StdMutex<T> {
    type Guard = StdMutexGuard<'_, T>;

    fn lock(&self) -> Self::Guard {
        self.lock().unwrap()
    }

}
