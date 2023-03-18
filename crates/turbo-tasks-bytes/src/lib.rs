mod bytes;
mod stream;

pub use crate::{
    bytes::{Bytes, BytesVc},
    stream::{Stream, StreamRead, StreamWrite},
};

pub fn register() {
    turbo_tasks::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
