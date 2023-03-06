# v0.8.0

- Add [DEBUG](DEBUG.md) docs
- Upgrade `sysinfo` from 0.27.1 -> 0.28.0
- Increase capacity of certain objects in minifilter

# v0.7.0

- Refactor minifilter
- Improve printing speed, for [example](src/bin/minifilter.rs)
- General stability improvements
- Upgrade `windows-rs` 0.43.0 -> 0.44.0
- Add rough [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) to `IPR` messages and compare them
  using the same
- Better handling of `exepath` for `IOMessage` IRP

# v0.6.0

- Fix an issue of floating point operations in the kernel driver
- Performance improvements

# v0.5.5

- Upgrade `C` standard to `C11`
- General stability improvements around IRQL, DriverEntry, etc
- Update `sysinfo` to `0.27.1`

# v0.5.0

- Replace `ZwClose` with `FltClose` in minifilter to solve potential memory leak
- Remove unused dependencies and add categories to `Cargo.toml`
- Vastly improve documentation
- Refactor code to be more readable and conscience

# v0.4.0

- Improve performance even further
- Add `#[inline]` calls to all functions
- Remove `x86`, `arm` and `arm64` targets from minifilter
- Upgrade to `c++20` standard for minifilter
- Reduce waiting time, for [example](src/bin/minifilter.rs)

# v0.3.0

- Improve performance of minifilter by using `-O2` and `-LTO` alongside release build
- Improve performance of [example](src/bin/minifilter.rs) by not locking and releasing IO
- Refactor and reformat minifilter
- Stop using debug libraries for minifilter
- Update readme and add changelog
- Add LICENSE