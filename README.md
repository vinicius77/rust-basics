### Basic Command Line

```
cargo run
```

```
cargo build
```

- Hot Reload

```
cargo watch -x run
```

- Quiet Mode

```
cargo watch -q -c -x 'run -q'
```

### Crate Attribute

- Silence unused warnings while learning (added in the top of the file)

```
#![allow(unused)]
```
