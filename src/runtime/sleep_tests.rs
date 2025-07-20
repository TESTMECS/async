#[cfg(test)]
mod tests {
    use crate::runtime::sleep::Sleep;
    use std::time::Duration;

    #[test]
    fn test_sleep_creation() {
        let _sleep = Sleep::new(Duration::from_millis(100));
        // Just verify creation succeeds - we can't access private fields
    }

    #[test]
    fn test_sleep_immediate_ready() {
        use std::task::{Context, Poll};
        use std::pin::Pin;
        use crate::runtime::waker::create_raw_waker;
        use std::task::Waker;
        
        let mut sleep = Sleep::new(Duration::from_nanos(1)); // Very short duration
        std::thread::sleep(Duration::from_millis(1)); // Ensure it's past
        
        let raw_waker = create_raw_waker();
        let waker = unsafe { Waker::from_raw(raw_waker) };
        let mut context = Context::from_waker(&waker);
        
        let result = Pin::new(&mut sleep).poll(&mut context);
        assert!(matches!(result, Poll::Ready(())));
    }

    #[test]
    fn test_sleep_pending() {
        use std::task::{Context, Poll};
        use std::pin::Pin;
        use crate::runtime::waker::create_raw_waker;
        use std::task::Waker;
        
        let mut sleep = Sleep::new(Duration::from_secs(1)); // Long duration
        
        let raw_waker = create_raw_waker();
        let waker = unsafe { Waker::from_raw(raw_waker) };
        let mut context = Context::from_waker(&waker);
        
        let result = Pin::new(&mut sleep).poll(&mut context);
        assert!(matches!(result, Poll::Pending));
    }

    #[test]
    fn test_sleep_becomes_ready() {
        use std::task::{Context, Poll};
        use std::pin::Pin;
        use crate::runtime::waker::create_raw_waker;
        use std::task::Waker;
        
        let mut sleep = Sleep::new(Duration::from_millis(50));
        
        let raw_waker = create_raw_waker();
        let waker = unsafe { Waker::from_raw(raw_waker) };
        let mut context = Context::from_waker(&waker);
        
        // Should be pending initially
        let result = Pin::new(&mut sleep).poll(&mut context);
        assert!(matches!(result, Poll::Pending));
        
        // Wait for the duration to pass
        std::thread::sleep(Duration::from_millis(60));
        
        // Should be ready now
        let result = Pin::new(&mut sleep).poll(&mut context);
        assert!(matches!(result, Poll::Ready(())));
    }

    #[test]
    fn test_multiple_sleep_durations() {
        let _short_sleep = Sleep::new(Duration::from_millis(10));
        let _long_sleep = Sleep::new(Duration::from_millis(100));
        
        // Just verify both can be created - we can't access private fields
    }
}
