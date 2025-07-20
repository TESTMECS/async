#[cfg(test)]
mod tests {
    use crate::runtime::reciever::TcpReceiver;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_tcp_receiver_creation() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let _ = stream.write_all(b"test response");
            }
        });

        let stream = TcpStream::connect(addr).unwrap();
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let receiver = TcpReceiver {
            stream: arc_stream,
            buffer: Vec::new(),
        };

        assert!(receiver.buffer.is_empty());
    }

    #[test]
    fn test_tcp_receiver_with_existing_buffer() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                drop(stream);
            }
        });

        thread::sleep(Duration::from_millis(10));

        let stream = TcpStream::connect(addr).unwrap();
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let existing_data = b"existing ";
        let receiver = TcpReceiver {
            stream: arc_stream,
            buffer: existing_data.to_vec(),
        };

        assert_eq!(receiver.buffer, existing_data);
    }

    #[test]
    fn test_tcp_receiver_large_buffer() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        thread::spawn(move || {
            let _ = listener.accept();
        });

        thread::sleep(Duration::from_millis(10));

        let stream = TcpStream::connect(addr).unwrap();
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let large_buffer = vec![0x42u8; 8192];
        let receiver = TcpReceiver {
            stream: arc_stream,
            buffer: large_buffer.clone(),
        };

        assert_eq!(receiver.buffer.len(), 8192);
        assert_eq!(receiver.buffer, large_buffer);
    }
}
