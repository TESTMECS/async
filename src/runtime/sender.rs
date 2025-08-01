use std::{
    future::Future,
    io::{self, Write},
    net::{TcpStream, Shutdown},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

pub struct TcpSender {
    pub stream: Arc<Mutex<TcpStream>>,
    pub buffer: Vec<u8>,
}
impl Future for TcpSender {
    type Output = io::Result<()>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut stream = match self.stream.try_lock() {
            Ok(stream) => stream,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };
        stream.set_nonblocking(true)?;
        match stream.write_all(&self.buffer) {
            Ok(_) => {
                // Shutdown the write side of the connection to signal we're done sending
                let _ = stream.shutdown(Shutdown::Write);
                Poll::Ready(Ok(()))
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
