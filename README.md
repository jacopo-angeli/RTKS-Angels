# `bern-rtos/template/cortex-m`

> A template for building applications for ARM Cortex-M microcontrollers using Bern RTOS.

## Dependencies

To build embedded programs using this template you'll need:

- Rust nightly toolchain. e.g. `rustup default nightly`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

# Attribution

This template is based on [`cortex-m-quickstart`](https://github.com/rust-embedded/cortex-m-quickstart).
