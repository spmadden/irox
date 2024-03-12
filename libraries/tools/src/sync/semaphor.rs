// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, TryLockError};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LockError {
    Poisoned,
    NotEnoughAvailable,
    RequestExceedsMaxCapacity,
    WouldBlock,
}

struct SemaphoreInner {
    max: u64,
    ro_available: AtomicU64,
    available: Mutex<u64>,
    waiter: Condvar,
}

pub struct SemaphoreLock<'a> {
    lock: &'a Semaphore,
    owned: u64,
}

impl<'a> Drop for SemaphoreLock<'a> {
    fn drop(&mut self) {
        if self.owned > 0 {
            let _ = self.lock.release(self.owned);
            self.owned = 0;
        }
    }
}

pub struct Semaphore {
    inner: Arc<SemaphoreInner>,
}
impl Clone for Semaphore {
    fn clone(&self) -> Self {
        Semaphore {
            inner: self.inner.clone(),
        }
    }
}

impl Semaphore {
    pub fn new(count: u64) -> Self {
        Semaphore {
            inner: Arc::new(SemaphoreInner {
                max: count,
                ro_available: AtomicU64::new(count),
                available: Mutex::new(count),
                waiter: Condvar::new(),
            }),
        }
    }

    pub fn take_one(&self) -> Result<SemaphoreLock, LockError> {
        self.take(1)
    }

    pub fn take(&self, count: u64) -> Result<SemaphoreLock, LockError> {
        if count > self.inner.max {
            return Err(LockError::RequestExceedsMaxCapacity);
        }

        let Ok(mut lock) = self.inner.available.lock() else {
            return Err(LockError::Poisoned);
        };
        if *lock < count {
            let Ok(mut lock) = self.inner.waiter.wait_while(lock, |v| *v < count) else {
                return Err(LockError::Poisoned);
            };
            if *lock < count {
                return Err(LockError::NotEnoughAvailable);
            }
            *lock -= count;
            self.inner.ro_available.store(*lock, Ordering::Relaxed);
            return Ok(SemaphoreLock {
                lock: self,
                owned: count,
            });
        }

        *lock -= count;
        self.inner.ro_available.store(*lock, Ordering::Relaxed);

        Ok(SemaphoreLock {
            lock: self,
            owned: count,
        })
    }

    pub fn release(&self, count: u64) -> Result<(), LockError> {
        let Ok(mut lock) = self.inner.available.lock() else {
            return Err(LockError::Poisoned);
        };
        *lock += count;
        self.inner.ro_available.store(*lock, Ordering::Relaxed);
        self.inner.waiter.notify_one();
        Ok(())
    }

    pub fn try_take(&self, count: u64) -> Result<SemaphoreLock, LockError> {
        if count > self.inner.max {
            return Err(LockError::RequestExceedsMaxCapacity);
        }

        if count > self.inner.ro_available.load(Ordering::Relaxed) {
            return Err(LockError::NotEnoughAvailable);
        }

        let lock = self.inner.available.try_lock();
        let mut val = match lock {
            Ok(v) => v,
            Err(e) => {
                return Err(match e {
                    TryLockError::Poisoned(_) => LockError::Poisoned,
                    TryLockError::WouldBlock => LockError::WouldBlock,
                })
            }
        };
        if count <= *val {
            *val -= count;
            self.inner.ro_available.store(*val, Ordering::Relaxed);
            self.inner.waiter.notify_one();
            return Ok(SemaphoreLock {
                lock: self,
                owned: count,
            });
        }
        Err(LockError::NotEnoughAvailable)
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::LockError;

    #[test]
    pub fn test() -> Result<(), LockError> {
        Ok(())
    }
}
