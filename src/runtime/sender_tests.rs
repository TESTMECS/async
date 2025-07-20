#[cfg(test)]
mod tests {
    use crate::runtime::sender::TcpSender;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_tcp_sender_creation() {
        // Create a mock TcpStream by connecting to a test server
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer);
                let _ = stream.write_all(b"response");
            }
        });

        let stream = TcpStream::connect(addr).unwrap();
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let sender = TcpSender {
            stream: arc_stream,
            buffer: b"test data".to_vec(),
        };

        // Just verify creation succeeded
        assert_eq!(sender.buffer, b"test data");
    }

    #[test]
    fn test_tcp_sender_with_empty_buffer() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        thread::spawn(move || {
            let _ = listener.accept();
        });

        thread::sleep(Duration::from_millis(10));

        let stream = TcpStream::connect(addr).unwrap();
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let sender = TcpSender {
            stream: arc_stream,
            buffer: Vec::new(), // Empty buffer
        };

        // Verify creation with empty buffer
        assert!(sender.buffer.is_empty());
    }

    #[test]
    fn test_tcp_sender_large_buffer() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        thread::spawn(move || {
            let _ = listener.accept();
        });

        thread::sleep(Duration::from_millis(10));

        let stream = TcpStream::connect(addr).unwrap();
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let large_data = vec![0x42u8; 5000]; // 5KB of data
        let sender = TcpSender {
            stream: arc_stream,
            buffer: large_data.clone(),
        };

        // Verify creation with large buffer
        assert_eq!(sender.buffer.len(), 5000);
        assert_eq!(sender.buffer, large_data);
    }
}
