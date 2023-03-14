use std::process;

use is_container::is_container;

fn main() {
    if !is_container() {
        process::exit(2);
    }
}
