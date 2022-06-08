# is-laptop

A rust crate for checking if the device is a laptop or not.

## Supported platforms

* Windows _(query through WMI)_

## Usage

Add the crate to your dependencies:

```toml
[dependencies]
is-laptop = "*"
```

Check if the device is a laptop

```rust
let isLaptop: bool = is_laptop::check();
```

## License

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
