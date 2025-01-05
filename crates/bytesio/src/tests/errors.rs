use scuffle_future_ext::FutureExt;

use crate::bytesio_errors::BytesIOError;

#[tokio::test]
async fn test_timeout_error_display() {
    let err = async {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    .with_timeout(std::time::Duration::from_millis(100))
    .await
    .unwrap_err();
    let bytes_io_error = BytesIOError::from(err);
    assert_eq!(bytes_io_error.to_string(), "timeout");

    let bytes_io_error = BytesIOError::ClientClosed;
    assert_eq!(bytes_io_error.to_string(), "client closed");
}

#[test]
fn test_bytesio_error_display() {
    let bytes_io_error = BytesIOError::ClientClosed;
    assert_eq!(bytes_io_error.to_string(), "client closed");
}
