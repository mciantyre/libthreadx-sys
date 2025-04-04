# libthreadx-sys

libthreadx-sys provides Rust bindings for [ThreadX][threadx], the real-time
operating system (RTOS). The package can build ThreadX for various host and MCU
ports. You can also use this package along with your pre-built ThreadX library.

For usage information, see the API documentation. The rest of this README
describes how to use this package in your project and how to customize the
build.

[threadx]: https://github.com/eclipse-threadx/threadx

## Usage

To use this package, include the package into your project's `Cargo.toml`. This
package isn't yet on crates.io, so use a git dependency.

You'll need a ThreadX static library to link against. There's two ways to obtain
that library:

1. Build ThreadX from source when building libthreadx-sys.
2. Reference a pre-built ThreadX static library in your build.

### Build from source

By default, libthreadx-sys will automatically build ThreadX from source for your
target port. This requires a C (cross) compiler.

The table below suggests where to you can obtain a C cross compiler for your
embedded target. If your ThreadX application runs on your host (i.e. Linux,
Windows), consult your operating system's documentation for a recommended C
compilers.

| ThreadX port              | Recommended cross compiler   |
|---------------------------|------------------------------|
| ARMv6-M, ARMv7-M, ARMv8-M | [ARM GNU Toolchain][gcc-arm] |

[gcc-arm]: https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain

Once you've installed your compiler, `cargo build` will automatically build
ThreadX for your host or target.

```bash
cargo build  # Targets your host
cargo build --target=thumbv7m-none-eabi  # Targets a ARMv7-M MCU.
```

### Reference a pre-built static library

If you have a pre-built ThreadX library, override the libthreadx-sys build
script and link against that pre-built library. For more information, see
[overriding build scripts][build-script-override] in the Cargo book.

[build-script-override]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts

The example below shows how to override the ThreadX library for a ARMv7-M
target.

```toml
[target.thumbv7m-none-eabi.threadx]
# TODO: change to reflect your archive's name.
rustc-link-lib = ["threadx-thumbv7m-none-eabi"]
rustc-link-search = ["path/to/that/directory"]
# TODO: include other configurations that customize Rust bindings
rustc-cfg = []
```

For convenience, this project provides pre-built ThreadX libraries for all
supported ports. To use these artifacts, see the releases section of the
repository. This project also provides a Cargo configuration that you can use in
your projects; see `.cargo/prebuilt.toml`.

## Customizing ThreadX

ThreadX supports many build customizations. Some configurations, like
`TX_ENABLE_STACK_CHECKING`, only change ThreadX internals without affecting the
ABI. When building from source, set these kinds of configurations in a `CFLAGS`
environment variable during the build.

Other configurations, like `TX_THREAD_ENABLE_PERFORMANCE_INFO`, change ThreadX
internals *and* affect the ABI. When specifying these kinds of configurations,
you must set a Rust configuration. The rest of this section talks about which
configurations are supported for the Rust bindings.

### Port / architecture

The target port will automatically be selected from the Rust build target. This
is true if you're building ThreadX from source, or if you're using a pre-built
version of ThreadX.

### Performance info

Enable performance information using conditional compilation options. Note that
these are *not* Cargo features. See
[here](https://doc.rust-lang.org/reference/conditional-compilation.html) for
more information.

If you're building ThreadX from source, then these Rust configurations will
automatically enable the corresponding ThreadX definition; you should not use
`CFLAGS` to enable these configurations. For example, if your build includes
`tx_thread_enable_performance_info`, then the libthreadx-sys build script will
automatically include `-DTX_THREAD_ENABLE_PERFORMANCE_INFO` when compiling
ThreadX.

If you're referencing a pre-built ThreadX library, then you must specify the
configurations that match your pre-built library. For example, if your pre-built
ThreadX artifact was built with thread performance information, then separately
set `tx_thread_enable_performance_info` as a configuration.

For the list of performance info configurations, consult this package's
`Cargo.toml` file, scanning for the checked configurations.

### Disable notify callbacks

The `tx_disable_notify_callbacks` configuration behaves similarly to performance
info configurations. If you build ThreadX from source, then the Rust
configuration will enable the ThreadX `TX_DISABLE_NOTIFY_CALLBACKS` definition.

If you're referencing a pre-built ThreadX library, then you must specify the
configuration yourself.

## Build script outputs

If the libthreadx-sys build script executes, it sets metadata that can be used
by immediate dependencies. Dependents access this metadata through environment
variables. See the [`links` manifest key][cargo-links] documentation for more
information.

`DEP_THREADX_COMMON_INCLUDE` is a path to a directory containing `tx_api.h` and
all other common headers. Consider using this as an include path if you're
compiling C code.

`DEP_THREADX_PORT_INCLUDE` is a path to a directory containing `tx_port.h`.
The contents of the header vary by port. Consider using this as an include
path if you're compiling C code.

`DEP_THREADX_PORT` is the specific port selected for the build. It's selected
by checking the build target. For example, the port could be `"cortex_m3"` when
building for an ARMv7-M target.

If you're [overriding the build script][build-script-override], you're
encouraged to set this same metadata in your override configuration.

[cargo-links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key

## Alternatives

libthreadx-sys is not the only ThreadX Rust binding project. Check out

- [threadx-sys] (available on crates.io)
- [threadx-experiments]

for other approaches. Some differences between these solutions and
libthreadx-sys:

- libthreadx-sys does not generate Rust bindings during the build.
- libthreadx-sys can link against custom ThreadX builds.

[threadx-sys]: https://docs.rs/crate/threadx-sys
[threadx-experiments]: https://github.com/ferrous-systems/threadx-experiments

## License

libthreadx-sys is MIT licensed. See [LICENSE](./LICENSE) for more information.

libthreadx-sys incorporates ThreadX API documentation from the
[eclipse-threadx/rtos-docs] repository. libthreadx-sys vendors this
documentation in files matching `src/doc/*.md`. The associated MIT license is
at `src/doc/LICENSE`.

libthreadx-sys includes ThreadX source code from the `threadx` directory.
See `threadx/LICENSE.txt` for ThreadX's MIT license.

[eclipse-threadx/rtos-docs]: https://github.com/eclipse-threadx/rtos-docs/tree/71a7c00f369b9cd648f73dfddb0d5ca6d5a1b855
