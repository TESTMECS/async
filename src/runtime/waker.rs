use std::task::{RawWaker, RawWakerVTable};

static VTABLE: RawWakerVTable = RawWakerVTable::new(my_clone, my_wake, my_wake_by_ref, my_drop);

unsafe fn my_clone(_raw_waker: *const ()) -> RawWaker {
    // Create a new box with the same value to avoid double-free
    let data = Box::into_raw(Box::new(42u32));
    RawWaker::new(data as *const (), &VTABLE)
}

unsafe fn my_wake(raw_waker: *const ()) {
    if !raw_waker.is_null() {
        unsafe { drop(Box::from_raw(raw_waker as *mut u32)); }
    }
}

unsafe fn my_wake_by_ref(_raw_waker: *const ()) {
    // No-op for wake_by_ref - don't drop the data
}

unsafe fn my_drop(raw_waker: *const ()) {
    if !raw_waker.is_null() {
        unsafe { drop(Box::from_raw(raw_waker as *mut u32)); }
    }
}

pub fn create_raw_waker() -> RawWaker {
    let data = Box::into_raw(Box::new(42u32));
    RawWaker::new(data as *const (), &VTABLE)
}
