#![cfg(feature = "full")]
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

#[cfg_attr(target_os = "wasi", ignore = "FIXME: empty poll in park")]
#[tokio::test]
async fn empty_read_is_cooperative() {
    tokio::select! {
        biased;

        _ = async {
            loop {
                let mut buf = [0u8; 4096];
                let _ = tokio::io::empty().read(&mut buf).await;
            }
        } => {},
        _ = tokio::task::yield_now() => {}
    }
}

#[cfg_attr(target_os = "wasi", ignore = "FIXME: empty poll in park")]
#[tokio::test]
async fn empty_buf_reads_are_cooperative() {
    tokio::select! {
        biased;

        _ = async {
            loop {
                let mut buf = String::new();
                let _ = tokio::io::empty().read_line(&mut buf).await;
            }
        } => {},
        _ = tokio::task::yield_now() => {}
    }
}
