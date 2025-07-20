use std_async::runtime::executor::Executor;
use std_async::runtime::sleep::Sleep;
use std_async::data::data_layer::Data;
use std_async::runtime::{sender::TcpSender, reciever::TcpReceiver};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[test]
fn test_complete_async_executor_workflow() {
    let mut executor = Executor::new();
    let mut results = Vec::new();
    
    // Spawn multiple async tasks
    for i in 0..5 {
        let rx = executor.spawn(async move {
            Sleep::new(Duration::from_millis(10 * i)).await;
            i * 2
        });
        results.push(rx);
    }
    
    // Run executor until all tasks complete
    let mut completed = 0;
    let mut values = Vec::new();
    
    for _ in 0..1000 {
        executor.poll();
        
        for rx in &results {
            if let Ok(value) = rx.try_recv() {
                values.push(value);
                completed += 1;
            }
        }
        
        if completed >= 5 {
            break;
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    assert_eq!(completed, 5);
    values.sort();
    assert_eq!(values, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_data_serialization_with_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    
    let test_data = Data {
        field1: 12345,
        field2: 6789,
        field3: "Integration Test".to_string(),
    };
    
    let test_data_clone = Data {
        field1: test_data.field1,
        field2: test_data.field2,
        field3: test_data.field3.clone(),
    };
    
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            // Read the serialized data
            let mut buffer = Vec::new();
            let mut temp_buf = [0; 1024];
            
            loop {
                match stream.read(&mut temp_buf) {
                    Ok(0) => break,
                    Ok(n) => buffer.extend_from_slice(&temp_buf[..n]),
                    Err(_) => break,
                }
            }
            
            // Deserialize and verify
            let mut cursor = std::io::Cursor::new(buffer.as_slice());
            let received_data = Data::deserialize(&mut cursor).unwrap();
            
            assert_eq!(received_data.field1, test_data_clone.field1);
            assert_eq!(received_data.field2, test_data_clone.field2);
            assert_eq!(received_data.field3, test_data_clone.field3);
            
            // Send response
            let _ = stream.write_all(b"Data received successfully");
        }
    });
    
    thread::sleep(Duration::from_millis(10));
    
    let stream = TcpStream::connect(addr).unwrap();
    let arc_stream = Arc::new(Mutex::new(stream));
    
    let mut executor = Executor::new();
    
    // Send data
    let serialized_data = test_data.serialize().unwrap();
    let arc_stream_clone = arc_stream.clone();
    let send_rx = executor.spawn(async move {
        TcpSender {
            stream: arc_stream_clone,
            buffer: serialized_data,
        }.await
    });
    
    // Receive response
    let recv_rx = executor.spawn(async move {
        TcpReceiver {
            stream: arc_stream,
            buffer: Vec::new(),
        }.await
    });
    
    // Run executor until completion
    let mut send_complete = false;
    let mut recv_complete = false;
    
    for _ in 0..1000 {
        executor.poll();
        
        if !send_complete {
            if let Ok(result) = send_rx.try_recv() {
                assert!(result.is_ok());
                send_complete = true;
            }
        }
        
        if !recv_complete {
            if let Ok(result) = recv_rx.try_recv() {
                let response = result.unwrap();
                let response_str = String::from_utf8(response).unwrap();
                assert_eq!(response_str, "Data received successfully");
                recv_complete = true;
            }
        }
        
        if send_complete && recv_complete {
            break;
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    assert!(send_complete, "Send operation should complete");
    assert!(recv_complete, "Receive operation should complete");
}

#[test]
fn test_concurrent_async_operations() {
    let mut executor = Executor::new();
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent sleep operations with different durations
    let durations = vec![50, 30, 20, 40, 10];
    for (i, &duration) in durations.iter().enumerate() {
        let rx = executor.spawn(async move {
            let start = std::time::Instant::now();
            Sleep::new(Duration::from_millis(duration)).await;
            let elapsed = start.elapsed();
            (i, elapsed)
        });
        handles.push(rx);
    }
    
    // Collect results
    let mut results = Vec::new();
    let mut completed = 0;
    
    for _ in 0..2000 {
        executor.poll();
        
        for rx in &handles {
            if let Ok((id, elapsed)) = rx.try_recv() {
                results.push((id, elapsed));
                completed += 1;
            }
        }
        
        if completed >= durations.len() {
            break;
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    assert_eq!(completed, durations.len());
    
    // Verify that tasks completed in roughly the right order
    // (shorter sleeps should complete before longer ones)
    results.sort_by_key(|(_, elapsed)| *elapsed);
    
    // The task with 10ms sleep should complete first (id=4)
    // The task with 50ms sleep should complete last (id=0)
    assert_eq!(results[0].0, 4); // 10ms sleep
    assert_eq!(results[4].0, 0); // 50ms sleep
}

#[test]
fn test_executor_with_complex_async_workflow() {
    let mut executor = Executor::new();
    
    // Create a complex workflow that involves multiple sleep operations
    let rx = executor.spawn(async {
        let mut total = 0;
        
        // First phase: short sleeps
        for i in 1..=3 {
            Sleep::new(Duration::from_millis(10)).await;
            total += i;
        }
        
        // Second phase: longer sleep
        Sleep::new(Duration::from_millis(50)).await;
        total *= 2;
        
        // Third phase: more operations
        for i in 1..=2 {
            Sleep::new(Duration::from_millis(5)).await;
            total += i * 10;
        }
        
        total
    });
    
    let start = std::time::Instant::now();
    
    // Run until completion
    let mut result = None;
    for _ in 0..2000 {
        executor.poll();
        
        if let Ok(value) = rx.try_recv() {
            result = Some(value);
            break;
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    let elapsed = start.elapsed();
    
    // Verify result: (1+2+3) * 2 + (1*10 + 2*10) = 6 * 2 + 30 = 42
    assert_eq!(result, Some(42));
    
    // Verify timing: should take at least 80ms (3*10 + 50 + 2*5)
    assert!(elapsed >= Duration::from_millis(70)); // Some tolerance
}

#[test]
fn test_error_handling_in_async_context() {
    let mut executor = Executor::new();
    
    // Test that we can handle errors in async operations
    let rx = executor.spawn(async {
        // Simulate an operation that might fail
        Sleep::new(Duration::from_millis(20)).await;
        
        // Try to parse an invalid number
        let result: Result<i32, _> = "not_a_number".parse();
        match result {
            Ok(num) => format!("Parsed: {}", num),
            Err(_) => "Failed to parse".to_string(),
        }
    });
    
    // Run until completion
    let mut result = None;
    for _ in 0..500 {
        executor.poll();
        
        if let Ok(value) = rx.try_recv() {
            result = Some(value);
            break;
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    assert_eq!(result, Some("Failed to parse".to_string()));
}
