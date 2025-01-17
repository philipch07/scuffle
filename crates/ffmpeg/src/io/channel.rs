use std::sync::{Arc, Mutex};

use bytes::{Buf, BytesMut};

/// A wrapper around a channel that implements `std::io::Read` and
/// `std::io::Write`. The wrapper allows for the channel to be used with the
/// `Input` and `Output` structs.
#[derive(Debug, Clone)]
pub struct ChannelCompat<T: Send> {
    /// I am unsure if the mutex is needed here. I do not think it is, but I am
    /// not sure. FFmpeg might require the IO to be synchronized, but I do not
    /// think it does.
    inner: Arc<Mutex<T>>,
    total: usize,
    pkt_idx: usize,
    buffer: BytesMut,
}

impl<T: Send> ChannelCompat<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            total: 0,
            pkt_idx: 0,
            buffer: BytesMut::new(),
        }
    }
}

pub trait ChannelCompatRecv: Send {
    type Data: AsRef<[u8]>;

    fn channel_recv(&mut self) -> Option<Self::Data>;

    fn into_compat(self) -> ChannelCompat<Self>
    where
        Self: Sized,
    {
        ChannelCompat::new(self)
    }
}

pub trait ChannelCompatSend: Send {
    type Data: From<Vec<u8>>;

    fn channel_send(&mut self, data: Self::Data) -> bool;

    fn into_compat(self) -> ChannelCompat<Self>
    where
        Self: Sized,
    {
        ChannelCompat::new(self)
    }
}

#[cfg(feature = "tokio-channel")]
impl<D: AsRef<[u8]> + Send> ChannelCompatRecv for tokio::sync::mpsc::Receiver<D> {
    type Data = D;

    fn channel_recv(&mut self) -> Option<Self::Data> {
        self.blocking_recv()
    }
}

#[cfg(feature = "tokio-channel")]
impl<D: From<Vec<u8>> + Send> ChannelCompatSend for tokio::sync::mpsc::Sender<D> {
    type Data = D;

    fn channel_send(&mut self, data: Self::Data) -> bool {
        self.blocking_send(data).is_ok()
    }
}

#[cfg(feature = "tokio-channel")]
impl<D: AsRef<[u8]> + Send> ChannelCompatRecv for tokio::sync::mpsc::UnboundedReceiver<D> {
    type Data = D;

    fn channel_recv(&mut self) -> Option<Self::Data> {
        self.blocking_recv()
    }
}

#[cfg(feature = "tokio-channel")]
impl<D: From<Vec<u8>> + Send> ChannelCompatSend for tokio::sync::mpsc::UnboundedSender<D> {
    type Data = D;

    fn channel_send(&mut self, data: Self::Data) -> bool {
        self.send(data).is_ok()
    }
}

#[cfg(feature = "tokio-channel")]
impl<D: AsRef<[u8]> + Clone + Send> ChannelCompatRecv for tokio::sync::broadcast::Receiver<D> {
    type Data = D;

    fn channel_recv(&mut self) -> Option<Self::Data> {
        self.blocking_recv().ok()
    }
}

#[cfg(feature = "tokio-channel")]
impl<D: From<Vec<u8>> + Clone + Send> ChannelCompatSend for tokio::sync::broadcast::Sender<D> {
    type Data = D;

    fn channel_send(&mut self, data: Self::Data) -> bool {
        self.send(data).is_ok()
    }
}

#[cfg(feature = "crossbeam-channel")]
impl<D: AsRef<[u8]> + Send> ChannelCompatRecv for crossbeam_channel::Receiver<D> {
    type Data = D;

    fn channel_recv(&mut self) -> Option<Self::Data> {
        self.recv().ok()
    }
}

#[cfg(feature = "crossbeam-channel")]
impl<D: From<Vec<u8>> + Send> ChannelCompatSend for crossbeam_channel::Sender<D> {
    type Data = D;

    fn channel_send(&mut self, data: Self::Data) -> bool {
        self.send(data).is_ok()
    }
}

impl<D: AsRef<[u8]> + Send> ChannelCompatRecv for std::sync::mpsc::Receiver<D> {
    type Data = D;

    fn channel_recv(&mut self) -> Option<Self::Data> {
        self.recv().ok()
    }
}

impl<D: From<Vec<u8>> + Send> ChannelCompatSend for std::sync::mpsc::Sender<D> {
    type Data = D;

    fn channel_send(&mut self, data: Self::Data) -> bool {
        self.send(data).is_ok()
    }
}

impl<T: ChannelCompatRecv> std::io::Read for ChannelCompat<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.is_empty() {
            let data = match self.inner.lock().unwrap().channel_recv() {
                Some(data) => data,
                None => return Ok(0),
            };
            let data = data.as_ref();

            self.pkt_idx += 1;
            self.total += data.len();

            let min = std::cmp::min(buf.len(), data.len());
            buf[..min].copy_from_slice(&data[..min]);
            if min < data.len() {
                self.buffer.extend_from_slice(&data[min..]);
            }
            Ok(min)
        } else {
            let min = std::cmp::min(buf.len(), self.buffer.len());
            buf[..min].copy_from_slice(&self.buffer[..min]);
            self.buffer.advance(min);
            Ok(min)
        }
    }
}

impl<T: ChannelCompatSend> std::io::Write for ChannelCompat<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.inner.lock().unwrap().channel_send(buf.to_vec().into()) {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Unexpected EOF"));
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::io::{Read, Write};
    use std::sync::mpsc;
    use std::thread;

    use tokio::sync::broadcast;
    use tokio::sync::mpsc::unbounded_channel;

    use crate::io::channel::{ChannelCompat, ChannelCompatRecv, ChannelCompatSend};

    #[test]
    fn test_channel_compat_new() {
        let (tx, _rx) = mpsc::channel::<Vec<u8>>();
        let channel_compat = ChannelCompat::new(tx);

        assert!(
            channel_compat.inner.lock().is_ok(),
            "Inner Mutex should be initialized and lockable"
        );
        assert_eq!(channel_compat.total, 0, "Total should be initialized to 0");
        assert_eq!(channel_compat.pkt_idx, 0, "pkt_idx should be initialized to 0");
        assert!(channel_compat.buffer.is_empty(), "Buffer should be empty at initialization");
    }

    #[test]
    fn test_into_compat() {
        struct MockReceiver {
            data: Option<Vec<u8>>,
        }

        impl ChannelCompatRecv for MockReceiver {
            type Data = Vec<u8>;

            fn channel_recv(&mut self) -> Option<Self::Data> {
                self.data.take()
            }
        }

        let receiver = MockReceiver {
            data: Some(vec![1, 2, 3]),
        };
        let compat = receiver.into_compat();

        assert!(compat.inner.lock().is_ok(), "Inner Mutex should be initialized and lockable");
        assert_eq!(compat.total, 0, "Total should be initialized to 0");
        assert_eq!(compat.pkt_idx, 0, "pkt_idx should be initialized to 0");
        assert!(compat.buffer.is_empty(), "Buffer should be empty at initialization");
    }

    #[test]
    fn test_into_compat_send() {
        struct MockSender {
            sent_data: Vec<Vec<u8>>,
        }

        impl ChannelCompatSend for MockSender {
            type Data = Vec<u8>;

            fn channel_send(&mut self, data: Self::Data) -> bool {
                self.sent_data.push(data);
                true
            }
        }

        let sender = MockSender { sent_data: vec![] };
        let compat = sender.into_compat();

        assert!(compat.inner.lock().is_ok(), "Inner Mutex should be initialized and lockable");
        assert_eq!(compat.total, 0, "Total should be initialized to 0");
        assert_eq!(compat.pkt_idx, 0, "pkt_idx should be initialized to 0");
        assert!(compat.buffer.is_empty(), "Buffer should be empty at initialization");
    }

    #[test]
    fn test_mpsc_channel_send_recv() {
        let (tx, rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1);
        let tx_compat = ChannelCompat::new(tx);
        let rx_compat = ChannelCompat::new(rx);

        let success = tx_compat.inner.lock().unwrap().channel_send(vec![1, 2, 3]);
        assert!(success);

        let received = rx_compat.inner.lock().unwrap().channel_recv();
        assert_eq!(received, Some(vec![1, 2, 3]));

        // drop the sender to close the channel so .channel_recv() returns `None`
        drop(tx_compat);

        // channel is closed; second call to channel_recv should return None
        let received_none = rx_compat.inner.lock().unwrap().channel_recv();
        assert!(received_none.is_none());
    }

    #[test]
    fn test_read_none_case() {
        let (tx, rx) = crossbeam_channel::unbounded::<Vec<u8>>();
        let mut rx_compat = ChannelCompat::new(rx);
        drop(tx);
        let mut buffer = [0u8; 10];
        let result = rx_compat.read(&mut buffer);

        assert_eq!(
            result.unwrap(),
            0,
            "Read should return 0 bytes when channel_recv returns None"
        );
    }

    #[test]
    fn test_read_partial_fill_and_buffer_remainder() {
        let (tx, rx) = crossbeam_channel::unbounded::<Vec<u8>>();
        let mut rx_compat = ChannelCompat::new(rx);
        tx.send(vec![1, 2, 3, 4, 5]).unwrap();
        let mut buffer = [0u8; 3];
        let result = rx_compat.read(&mut buffer);

        assert_eq!(result.unwrap(), 3, "Read should return the buffer size when data exceeds it");
        assert_eq!(buffer, [1, 2, 3], "Buffer should contain the first part of the data");
        assert_eq!(
            rx_compat.buffer.as_ref(),
            [4, 5],
            "Remaining data should be stored in the internal buffer"
        );
        drop(tx);
    }

    #[test]
    fn test_read_from_buffer_when_not_empty() {
        let (tx, rx) = crossbeam_channel::unbounded::<Vec<u8>>();
        let mut rx_compat = ChannelCompat::new(rx);
        tx.send(vec![1, 2, 3, 4, 5]).unwrap();
        let mut buffer = [0u8; 3];
        let result = rx_compat.read(&mut buffer);

        assert_eq!(result.unwrap(), 3, "Read should return the buffer size when data exceeds it");
        assert_eq!(buffer, [1, 2, 3], "Buffer should contain the first part of the data");

        let mut buffer2 = [0u8; 2];
        let result2 = rx_compat.read(&mut buffer2);

        assert_eq!(
            result2.unwrap(),
            2,
            "Should read the remaining bytes from the internal buffer"
        );
        assert_eq!(
            buffer2,
            [4, 5],
            "Buffer should contain the remaining data from the internal buffer"
        );
        assert!(
            rx_compat.buffer.is_empty(),
            "Internal buffer should be empty after reading all data"
        );
        drop(tx);
    }

    #[test]
    fn test_tokio_mpsc_send_recv() {
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let mut rx_compat = rx.into_compat();
        let mut tx_compat = tx.into_compat();

        let handle = thread::spawn(move || {
            tx_compat.write_all(b"Hello MPSC").unwrap();
        });

        let mut buf = [0u8; 10];
        rx_compat.read_exact(&mut buf).unwrap();

        assert_eq!(&buf, b"Hello MPSC");
        handle.join().expect("Sender thread panicked");
    }

    #[test]
    fn test_tokio_unbounded_mpsc_send_recv() {
        let (tx, rx) = unbounded_channel::<Vec<u8>>();
        let mut rx_compat = rx.into_compat();
        let mut tx_compat = tx.into_compat();
        let handle = thread::spawn(move || {
            tx_compat.write_all(b"Hello Unbounded").unwrap();
        });
        let mut buf = [0u8; 15];
        rx_compat.read_exact(&mut buf).unwrap();

        assert_eq!(&buf, b"Hello Unbounded");

        handle.join().expect("Sender thread panicked");
    }

    #[test]
    fn test_tokio_broadcast_send_recv() {
        let (tx, rx1) = broadcast::channel::<Vec<u8>>(10);
        let mut rx_compat = rx1.into_compat();
        let mut tx_compat = tx.into_compat();
        let handle = thread::spawn(move || {
            tx_compat.write_all(b"Hello Broadcast").unwrap();
        });
        let mut buf = [0u8; 15];
        rx_compat.read_exact(&mut buf).unwrap();

        assert_eq!(&buf, b"Hello Broadcast");
        handle.join().expect("Sender thread panicked");
    }

    #[test]
    fn test_crossbeam_channel_send_recv() {
        let (tx, rx) = crossbeam_channel::unbounded::<Vec<u8>>();
        let tx_compat = ChannelCompat::new(tx);
        let rx_compat = ChannelCompat::new(rx);

        let success = tx_compat.inner.lock().unwrap().channel_send(vec![1, 2, 3]);
        assert!(success, "Sending should succeed when receiver is alive");

        let received = rx_compat.inner.lock().unwrap().channel_recv();
        assert_eq!(received, Some(vec![1, 2, 3]), "Should receive the same data that was sent");
    }

    #[test]
    fn test_write_unexpected_eof() {
        let (tx, rx) = crossbeam_channel::unbounded::<Vec<u8>>();
        let mut tx_compat = ChannelCompat::new(tx);
        drop(rx);
        let data = [1, 2, 3, 4, 5];
        let result = tx_compat.write(&data);

        assert!(result.is_err(), "Write should fail when the receiver is dropped");
        let error = result.unwrap_err();
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::UnexpectedEof,
            "Error kind should be UnexpectedEof"
        );
        assert_eq!(
            error.to_string(),
            "Unexpected EOF",
            "Error message should match the expected string"
        );
    }

    #[test]
    fn test_flush() {
        let (tx, _rx) = crossbeam_channel::unbounded::<Vec<u8>>();
        let mut tx_compat = ChannelCompat::new(tx);

        let result = tx_compat.flush();
        assert!(result.is_ok(), "Flush should succeed and return Ok(())");
    }
}
