use std_async::runtime::executor::Executor;
use std_async::runtime::sleep::Sleep;
use std::time::{Duration, Instant};

#[test]
fn bench_executor_many_tasks() {
    let mut executor = Executor::new();
    let start = Instant::now();
    let mut handles = Vec::new();
    
    // Spawn 1000 lightweight tasks
    for i in 0..1000 {
        let rx = executor.spawn(async move {
            // Very short computation
            i * 2
        });
        handles.push(rx);
    }
    
    let spawn_time = start.elapsed();
    println!("Spawned 1000 tasks in: {:?}", spawn_time);
    
    let start = Instant::now();
    let mut completed = 0;
    
    // Execute all tasks
    for _ in 0..10000 {
        executor.poll();
        
        for rx in &handles {
            if let Ok(_) = rx.try_recv() {
                completed += 1;
            }
        }
        
        if completed >= 1000 {
            break;
        }
    }
    
    let execution_time = start.elapsed();
    println!("Executed 1000 tasks in: {:?}", execution_time);
    
    assert_eq!(completed, 1000);
    
    // Performance assertions - should be quite fast
    assert!(spawn_time < Duration::from_millis(100));
    assert!(execution_time < Duration::from_millis(100));
}

#[test]
fn bench_sleep_precision() {
    let mut executor = Executor::new();
    let test_durations = vec![1, 5, 10, 20, 50, 100]; // milliseconds
    
    for &duration_ms in &test_durations {
        let start = Instant::now();
        let duration = Duration::from_millis(duration_ms);
        
        let rx = executor.spawn(async move {
            Sleep::new(duration).await;
            Instant::now()
        });
        
        let mut completed = false;
        for _ in 0..5000 {
            executor.poll();
            
            if let Ok(end_time) = rx.try_recv() {
                let actual_duration = end_time.duration_since(start);
                let difference = if actual_duration > duration {
                    actual_duration - duration
                } else {
                    duration - actual_duration
                };
                
                println!("Target: {:?}, Actual: {:?}, Difference: {:?}", 
                         duration, actual_duration, difference);
                
                // Allow some tolerance (up to 20ms difference for short sleeps)
                let tolerance = Duration::from_millis(20);
                assert!(difference < tolerance, 
                        "Sleep duration too inaccurate: target={:?}, actual={:?}, diff={:?}",
                        duration, actual_duration, difference);
                
                completed = true;
                break;
            }
            
            std::thread::sleep(Duration::from_millis(1));
        }
        
        assert!(completed, "Sleep for {}ms did not complete", duration_ms);
    }
}

#[test]
fn bench_concurrent_sleeps() {
    let mut executor = Executor::new();
    let start = Instant::now();
    let mut handles = Vec::new();
    
    // Start 100 concurrent sleep operations
    for i in 0..100 {
        let sleep_duration = Duration::from_millis(50 + (i % 10) as u64);
        let rx = executor.spawn(async move {
            let task_start = Instant::now();
            Sleep::new(sleep_duration).await;
            (i, task_start.elapsed())
        });
        handles.push(rx);
    }
    
    let mut results = Vec::new();
    let mut completed = 0;
    
    // Execute until all complete
    for _ in 0..10000 {
        executor.poll();
        
        for rx in &handles {
            if let Ok((id, duration)) = rx.try_recv() {
                results.push((id, duration));
                completed += 1;
            }
        }
        
        if completed >= 100 {
            break;
        }
        
        std::thread::sleep(Duration::from_millis(1));
    }
    
    let total_time = start.elapsed();
    println!("100 concurrent sleeps completed in: {:?}", total_time);
    
    assert_eq!(completed, 100);
    
    // All tasks should complete in roughly the same wall-clock time
    // (since they're concurrent), so total time should be close to the longest sleep
    assert!(total_time < Duration::from_millis(150)); // 50ms + some overhead
    
    // Individual tasks should have taken their expected time
    for (id, duration) in results {
        let expected = Duration::from_millis(50 + (id % 10) as u64);
        let tolerance = Duration::from_millis(30);
        assert!(duration >= expected.saturating_sub(tolerance) && 
                duration <= expected + tolerance,
                "Task {} took {:?}, expected around {:?}", id, duration, expected);
    }
}

#[test]
fn bench_memory_usage() {
    let mut executor = Executor::new();
    
    // Test that we can handle many tasks without excessive memory usage
    let mut handles = Vec::new();
    
    // Create a large number of tasks
    for i in 0..5000 {
        let rx = executor.spawn(async move {
            // Small sleep to make tasks persist for a while
            Sleep::new(Duration::from_millis(1)).await;
            i
        });
        handles.push(rx);
    }
    
    println!("Created 5000 tasks, queue length: {}", executor.polling.len());
    
    // Execute all tasks
    let start = Instant::now();
    let mut completed = 0;
    
    for _ in 0..50000 {
        executor.poll();
        
        for rx in &handles {
            if let Ok(_) = rx.try_recv() {
                completed += 1;
            }
        }
        
        if completed >= 5000 {
            break;
        }
        
        // Small delay to allow sleeps to complete
        if completed < 1000 {
            std::thread::sleep(Duration::from_nanos(100_000));
        }
    }
    
    let execution_time = start.elapsed();
    println!("Executed 5000 tasks in: {:?}", execution_time);
    println!("Final queue length: {}", executor.polling.len());
    
    assert_eq!(completed, 5000);
    assert_eq!(executor.polling.len(), 0); // All tasks should be done
    
    // Should complete in reasonable time
    assert!(execution_time < Duration::from_secs(2));
}
