#[cfg(test)]
mod tests {
    use crate::runtime::executor::Executor;
    use crate::runtime::sleep::Sleep;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_executor_creation() {
        let executor = Executor::new();
        assert_eq!(executor.polling.len(), 0);
    }

    #[test]
    fn test_executor_spawn_simple_future() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let rx = executor.spawn(async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            42
        });

        // Poll until completion
        for _ in 0..10 {
            executor.poll();
            if let Ok(result) = rx.try_recv() {
                assert_eq!(result, 42);
                assert_eq!(counter.load(Ordering::SeqCst), 1);
                return;
            }
        }
        
        panic!("Future did not complete");
    }

    #[test]
    fn test_executor_spawn_multiple_futures() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicU32::new(0));
        let mut receivers = Vec::new();

        // Spawn multiple futures
        for i in 0..5 {
            let counter_clone = counter.clone();
            let rx = executor.spawn(async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                i * 2
            });
            receivers.push(rx);
        }

        // Poll until all complete
        let mut completed = 0;
        for _ in 0..50 {
            executor.poll();
            
            for rx in &receivers {
                if let Ok(_) = rx.try_recv() {
                    completed += 1;
                }
            }
            
            if completed >= 5 {
                break;
            }
        }

        assert_eq!(counter.load(Ordering::SeqCst), 5);
        assert_eq!(completed, 5);
    }

    #[test]
    fn test_executor_with_sleep_future() {
        use crate::runtime::sleep::Sleep;
        
        let mut executor = Executor::new();
        let start = std::time::Instant::now();
        
        let rx = executor.spawn(async {
            Sleep::new(Duration::from_millis(50)).await;
            "completed"
        });

        // Poll until completion
        for _ in 0..1000 {
            executor.poll();
            if let Ok(result) = rx.try_recv() {
                let elapsed = start.elapsed();
                assert_eq!(result, "completed");
                assert!(elapsed >= Duration::from_millis(40)); // Some tolerance
                return;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        
        panic!("Sleep future did not complete");
    }

    #[test]
    fn test_waker_creation() {
        let executor = Executor::new();
        let waker = executor.create_waker();
        
        // Just ensure waker can be created and cloned
        let _cloned_waker = waker.clone();
    }

    #[test]
    fn test_executor_handles_pending_futures() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let rx = executor.spawn(async move {
            // This will be pending initially
            Sleep::new(Duration::from_millis(10)).await;
            counter_clone.fetch_add(1, Ordering::SeqCst);
            "done"
        });

        // First few polls should return pending
        for _ in 0..5 {
            executor.poll();
            assert!(rx.try_recv().is_err()); // Should still be pending
        }
        
        // Wait a bit and continue polling
        std::thread::sleep(Duration::from_millis(20));
        
        for _ in 0..10 {
            executor.poll();
            if let Ok(result) = rx.try_recv() {
                assert_eq!(result, "done");
                assert_eq!(counter.load(Ordering::SeqCst), 1);
                return;
            }
        }
        
        panic!("Future should have completed");
    }
}
