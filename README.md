# fsfilter-rs

[![Rust](https://github.com/sn99/fsfilter-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/sn99/fsfilter-rs/actions/workflows/rust.yml)

A rust library to monitor filesystem and more in windows

Prepared as part of ongoing thesis work at uni.

![shared_def](readme_resources/shared_def.png)

### MINIFILTER

See [MINIFILTER.md](MINIFILTER.md) for building the minifilter or just [right click install using the `.inf` file
provided in releases](https://github.com/sn99/fsfilter-rs/releases/latest/download/snFilter.zip).

**NOTE: By default it is built for Windows 10 and above**

**NOTE: Enable Loading of Test Signed Drivers by executing `Bcdedit.exe -set TESTSIGNING ON` in administrative cmd**

### RUNNING EXAMPLE

Use `cargo run --bin minifilter --release` to run the example application. The program starts to print the `IOMessage`
which is defined like:

```rust
#[repr(C)]
pub struct IOMessage {
    pub extension: [wchar_t; 12],
    pub file_id_vsn: c_ulonglong,
    pub file_id_id: [u8; 16],
    pub mem_sized_used: c_ulonglong,
    pub entropy: f64,
    pub pid: c_ulong,
    pub irp_op: c_uchar,
    pub is_entropy_calc: u8,
    pub file_change: c_uchar,
    pub file_location_info: c_uchar,
    pub filepathstr: String,
    pub gid: c_ulonglong,
    pub runtime_features: RuntimeFeatures,
    pub file_size: i64,
}
```

## PERFORMANCE

The performance of the minifilter doesn't really exceed `1%` of the CPU usage (I never saw it tickle even to 1% while
running scripts to make multiple temporary files). Although depending on you console if you try running
`cargo run --bin minifilter` you might see spikes reaching `1-3%` but that is because of the console itself (comment out
the `writeln!` in the bin example).

## LICENSE

This project is licensed under the terms of the [MIT license](LICENSE.md).

## ACKNOWLEDGEMENTS

- [RansomWatch](https://github.com/RafWu/RansomWatch)
- [SitinCloud](https://github.com/SitinCloud)
- [SubconsciousCompute](https://github.com/SubconsciousCompute)