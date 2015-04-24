# modules
Experimental library with high-level abstractions for
[emlib](https://github.com/RustyGecko/emlib) peripheral bindings.

## Building
The library needs to be built for the ARM Coretex M3:
```
$ cargo build --target thumbv7m-none-eabi
```

## Using
Add as a target-dependency in your `Cargo.toml`.
```toml
[target.thumbv7m-none-eabi.dependencies.modules]
git = "https://github.com/RustyGecko/modules.git"
```
