//! Check if the process is running inside a container.
//!
//! # Examples
//!
//! ```
//! use is_container::is_container;
//!
//! if is_container() {
//!     println!("Running inside a Docker container");
//! }
//! ```
//!
//! # Inspired
//!
//! - [How to determine if a process runs inside lxc/Docker?](https://stackoverflow.com/questions/20010199/how-to-determine-if-a-process-runs-inside-lxc-docker)
//! - [sindresorhus/is-inside-container](https://github.com/sindresorhus/is-inside-container)

#![feature(once_cell)]

use std::fs;
use std::sync::OnceLock;

static ONCE: OnceLock<bool> = OnceLock::new();

fn has_docker_env() -> bool {
    fs::metadata("/.dockerenv").is_ok()
}

fn has_container_env() -> bool {
    fs::metadata("/run/.containerenv").is_ok()
}

fn has_docker_cgroup() -> bool {
    match fs::read_to_string("/proc/1/cgroup") {
        Ok(contents) => contents.contains("docker") || contents.contains("lxc"),
        Err(_) => false,
    }
}

/// The main function provided by this crate. See crate documentation for more information.
pub fn is_container() -> bool {
    *ONCE.get_or_init(|| has_docker_env() || has_container_env() || has_docker_cgroup())
}