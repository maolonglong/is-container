//! Check if the process is running inside a container.
//!
//! # Examples
//!
//! ```no_run
//! use is_container::is_container;
//!
//! if is_container() {
//!     println!("Running inside a container");
//! }
//! ```
//!
//! # Inspired
//!
//! - [How to determine if a process runs inside lxc/Docker?](https://stackoverflow.com/questions/20010199/how-to-determine-if-a-process-runs-inside-lxc-docker)
//! - [sindresorhus/is-inside-container](https://github.com/sindresorhus/is-inside-container)

use std::fs;

use once_cell::sync::Lazy;

fn has_docker_env() -> bool {
    fs::metadata("/.dockerenv").is_ok()
}

fn has_container_env() -> bool {
    fs::metadata("/run/.containerenv").is_ok()
}

fn has_cgroup_v1() -> bool {
    fs::read_to_string("/proc/1/cgroup").map_or(false, |contents| {
        contents.contains("/docker/") || contents.contains("/lxc/")
    })
}

fn has_mountinfo() -> bool {
    fs::read_to_string("/proc/1/mountinfo").map_or(false, |contents| {
        // See https://man7.org/linux/man-pages/man5/proc.5.html
        //
        // The file contains lines of the form:
        // 36 35 98:0 /mnt1 /mnt2 rw,noatime master:1 - ext3 /dev/root rw,errors=continue
        // (1)(2)(3)   (4)   (5)      (6)      (7)   (8) (9)   (10)         (11)
        contents.lines().any(|line| {
            line.split_whitespace().nth(3).map_or(false, |mnt1| {
                mnt1.contains("/docker/") || mnt1.contains("/lxc/")
            })
        })
    })
}

/// The main function provided by this crate. See crate documentation for more information.
pub fn is_container() -> bool {
    static CACHED_RESULT: Lazy<bool> =
        Lazy::new(|| has_docker_env() || has_container_env() || has_mountinfo() || has_cgroup_v1());

    *CACHED_RESULT
}
