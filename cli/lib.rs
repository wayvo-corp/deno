#![allow(warnings)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod args;
pub mod auth_tokens;
pub mod cache;
pub mod cdp;
pub mod emit;
pub mod errors;
pub mod factory;
pub mod file_fetcher;
pub mod graph_util;
pub mod http_util;
pub mod js;
pub mod jsr;
pub mod lsp;
pub mod module_loader;
pub mod napi;
pub mod node;
pub mod npm;
pub mod ops;
pub mod resolver;
pub mod standalone;
pub mod tools;
pub mod tsc;
pub mod util;
pub mod version;
pub mod worker;

pub use crate::args::flags_from_vec;
pub use crate::args::DenoSubcommand;
pub use crate::args::Flags;
pub use crate::util::display;
pub use crate::util::v8::get_v8_flags_from_env;
pub use crate::util::v8::init_v8_flags;

pub use deno_core::anyhow::Context;
pub use deno_core::error::AnyError;
pub use deno_core::error::JsError;
pub use deno_core::futures::FutureExt;
pub use deno_core::unsync::JoinHandle;
use deno_runtime::colors;
pub use deno_runtime::fmt_errors::format_js_error;
pub use deno_runtime::tokio_util::create_and_run_current_thread_with_maybe_metrics;
pub use factory::CliFactory;
pub use std::env;
use std::env::current_exe;
use std::future::Future;
use std::path::PathBuf;

pub use deno_runtime;

// NOTE(bartlomieju): keep IDs in sync with `runtime/90_deno_ns.js` (search for `unstableFeatures`)
pub(crate) static UNSTABLE_GRANULAR_FLAGS: &[(
  // flag name
  &str,
  // help text
  &str,
  // id to enable it in runtime/99_main.js
  i32,
)] = &[
  (
    deno_runtime::deno_broadcast_channel::UNSTABLE_FEATURE_NAME,
    "Enable unstable `BroadcastChannel` API",
    1,
  ),
  (
    deno_runtime::deno_cron::UNSTABLE_FEATURE_NAME,
    "Enable unstable Deno.cron API",
    2,
  ),
  (
    deno_runtime::deno_ffi::UNSTABLE_FEATURE_NAME,
    "Enable unstable FFI APIs",
    3,
  ),
  (
    deno_runtime::deno_fs::UNSTABLE_FEATURE_NAME,
    "Enable unstable file system APIs",
    4,
  ),
  (
    deno_runtime::ops::http::UNSTABLE_FEATURE_NAME,
    "Enable unstable HTTP APIs",
    5,
  ),
  (
    deno_runtime::deno_kv::UNSTABLE_FEATURE_NAME,
    "Enable unstable Key-Value store APIs",
    6,
  ),
  (
    deno_runtime::deno_net::UNSTABLE_FEATURE_NAME,
    "Enable unstable net APIs",
    7,
  ),
  (
    "unsafe-proto",
    "Enable unsafe __proto__ support. This is a security risk.",
    // This number is used directly in the JS code. Search
    // for "unstableFeatures" to see where it's used.
    8,
  ),
  (
    deno_runtime::deno_webgpu::UNSTABLE_FEATURE_NAME,
    "Enable unstable `WebGPU` API",
    9,
  ),
  (
    deno_runtime::ops::worker_host::UNSTABLE_FEATURE_NAME,
    "Enable unstable Web Worker APIs",
    10,
  ),
];

pub(crate) fn unstable_exit_cb(feature: &str, api_name: &str) {
  eprintln!(
    "Unstable API '{api_name}'. The `--unstable-{}` flag must be provided.",
    feature
  );
  std::process::exit(70);
}

// TODO(bartlomieju): remove when `--unstable` flag is removed.
pub(crate) fn unstable_warn_cb(feature: &str, api_name: &str) {
  eprintln!(
      "⚠️  {}",
      colors::yellow(format!(
        "The `{}` API was used with `--unstable` flag. The `--unstable` flag is deprecated and will be removed in Deno 2.0. Use granular `--unstable-{}` instead.\nLearn more at: https://docs.deno.com/runtime/manual/tools/unstable_flags",
        api_name, feature
      ))
    );
}
