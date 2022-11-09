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
- Reduce wait time for [example](src/bin/minifilter.rs)

# v0.3.0

- Improve performance of minifilter by using `-O2` and `-LTO` alongside release build
- Improve performance of [example](src/bin/minifilter.rs) by not locking and releasing IO
- Refactor and reformat minifilter
- Stop using debug libraries for minifilter
- Update readme and add changelog
- Add LICENSE