#[cfg(test)]
mod tests {
    use crate::runtime::waker::create_raw_waker;
    use std::task::Waker;

    #[test]
    fn test_create_raw_waker() {
        let raw_waker = create_raw_waker();
        
        // Ensure we can create a Waker from the RawWaker
        let waker = unsafe { Waker::from_raw(raw_waker) };
        
        // Test that we can clone the waker
        let _cloned_waker = waker.clone();
    }

    #[test]
    fn test_waker_wake() {
        let raw_waker = create_raw_waker();
        let waker = unsafe { Waker::from_raw(raw_waker) };
        
        // Test wake operations (should not panic)
        waker.wake_by_ref();
        waker.wake();
    }

    #[test]
    fn test_waker_clone() {
        let raw_waker = create_raw_waker();
        let waker = unsafe { Waker::from_raw(raw_waker) };
        
        let cloned_waker = waker.clone();
        
        // Both should work without panicking
        waker.wake_by_ref();
        cloned_waker.wake();
    }

    #[test]
    fn test_raw_waker_creation() {
        let raw_waker1 = create_raw_waker();
        let raw_waker2 = create_raw_waker();
        
        // Both should be created successfully - we can't access private fields
        let _waker1 = unsafe { Waker::from_raw(raw_waker1) };
        let _waker2 = unsafe { Waker::from_raw(raw_waker2) };
    }

    #[test]
    fn test_multiple_waker_operations() {
        for _ in 0..10 {
            let raw_waker = create_raw_waker();
            let waker = unsafe { Waker::from_raw(raw_waker) };
            
            // Clone and wake multiple times
            let cloned = waker.clone();
            cloned.wake_by_ref();
            waker.wake();
        }
    }
}
