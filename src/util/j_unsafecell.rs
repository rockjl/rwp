/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::cell::UnsafeCell;

#[derive(Debug)]
pub(crate) struct JUnsafeCell<T> {
    raw: UnsafeCell<T>,
}
impl<T> JUnsafeCell<T> {
    pub(crate) fn new(t: T) -> Self {
        Self {
            raw: UnsafeCell::new(t),
        }
    }
    pub(crate) fn with<F: FnOnce(*const T) -> R, R>(&self, f: F) -> R {
        f(self.raw.get())
    }
    pub(crate) fn with_mut<F: FnOnce(*mut T) -> R, R>(&self, f: F) -> R {
        f(self.raw.get())
    }
    pub(crate) fn async_with_mut<F: FnOnce(*mut T) -> R, R>(&self, f: F) -> R{
        f(self.raw.get())
    }
}
unsafe impl<T> Send for JUnsafeCell<T> {}
unsafe impl<T> Sync for JUnsafeCell<T> {}