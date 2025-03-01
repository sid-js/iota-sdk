// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A general purpose IOTA client for interaction with the IOTA network (Tangle)
//!
//! High-level functions are accessible via the [`Client`][client::Client].
//!
//! ## Sending a block without a payload
//!  ```no_run
//! # use iota_sdk::client::{Client, Result};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = Client::builder()
//!    .with_node("http://localhost:14265")?
//!    .finish()
//!    .await?;
//!
//! let block = client
//!    .block()
//!    .finish()
//!    .await?;
//!
//! println!("Block sent {}", block.id());
//! # Ok(())}
//! ```

#[cfg(feature = "mqtt")]
macro_rules! lazy_static {
    ($init:expr => $type:ty) => {{
        static mut VALUE: Option<$type> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        INIT.call_once(|| unsafe { VALUE = Some($init) });
        unsafe { VALUE.as_ref() }.expect("failed to get lazy static value")
    }};
}

pub mod api;
pub mod builder;
pub mod constants;
pub mod core;
pub mod error;
pub mod node_api;
pub mod node_manager;
#[cfg(not(target_family = "wasm"))]
pub(crate) mod request_pool;
pub mod secret;
pub mod storage;
#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
pub mod stronghold;
pub mod utils;

#[cfg(feature = "mqtt")]
pub use self::node_api::mqtt;
pub use self::{
    builder::{ClientBuilder, NetworkInfo},
    core::*,
    error::*,
    node_api::core::routes::NodeInfoWrapper,
    utils::*,
};

#[cfg(feature = "mqtt")]
mod async_runtime {
    use std::sync::Mutex;

    use once_cell::sync::OnceCell;
    use tokio::runtime::Runtime;

    static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();

    pub(crate) fn spawn<F>(future: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().expect("failed to create Tokio runtime")));
        runtime.lock().expect("failed to lock the runtime.").spawn(future);
    }
}
