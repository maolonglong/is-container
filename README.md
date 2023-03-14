# is-container

Check if the process is running inside a container.

## Lib

```bash
$ cargo add is-container
```

```rust
use is_container::is_container;

if is_container() {
    println!("Running inside a Docker container");
}
```

## CLI

```bash
$ cargo install is-container
```

```bash
$ is-container

# Exits with code 0 if inside a container and 2 if not.
$ echo $?
2
```
