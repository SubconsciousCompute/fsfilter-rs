//! # minifilter-rs
//!
//! **Use `cargo doc --no-deps --document-private-items --open` to read Documentation**
//!
//! ## Table of Contents
//!
//! <details>
//!     <summary>Table of Contents</summary>
//!
//! - [Minifilter Driver](https://!github.com/sn99/fsfilter-rs#minifilter-driver)
//!     - [Building Driver](https://!github.com/sn99/fsfilter-rs#building-driver)
//!     - [Installing Driver](https://!github.com/sn99/fsfilter-rs#building-driver)
//!     - [Loading/Removing Driver](https://!github.com/sn99/fsfilter-rs#loadingremoving-driver)
//! - [Rust Application](https://!github.com/sn99/fsfilter-rs#rust-application)
//!     - [Building Rust App](https://!github.com/sn99/fsfilter-rs#building-rust-app)
//!     - [Running Rust App](https://!github.com/sn99/fsfilter-rs#running-rust-app)
//! - [What and the How](https://!github.com/sn99/fsfilter-rs#what-and-the-how)
//!
//! </details>
//!
//! ## Minifilter Driver
//!
//! ### Building Driver
//!
//! 1. Open `VS 2022`
//! 2. Goto `minifilter-rs -> minifilter -> RWatch.sln`
//! 3. Build solution in `Release` mode with `x64`
//!
//! **NOTE: Enable Loading of Test Signed Drivers by executing `Bcdedit.exe -set TESTSIGNING ON` in administrative cmd**
//!
//! ### Installing Driver
//!
//! 1. Open Powershell or command prompt as Administrator
//! 2. `RUNDLL32.EXE SETUPAPI.DLL,InstallHinfSection DefaultInstall 132 <path-to>\minifilter-rs\minifilter\x64\Debug\snFilter.inf`
//!
//! You should be able to see the driver at `"C:\Windows\System32\drivers\snFilter.sys"`
//!
//! ### Loading/Removing Driver
//!
//! 1. Open Powershell or command prompt as Administrator
//! 2. Start the driver using `sc start snFilter`, expected output:
//!     ```ignore
//!    SERVICE_NAME: snFilter
//!         TYPE               : 2  FILE_SYSTEM_DRIVER
//!         STATE              : 4  RUNNING
//!                                 (STOPPABLE, NOT_PAUSABLE, IGNORES_SHUTDOWN)
//!         WIN32_EXIT_CODE    : 0  (0x0)
//!         SERVICE_EXIT_CODE  : 0  (0x0)
//!         CHECKPOINT         : 0x0
//!         WAIT_HINT          : 0x0
//!         PID                : 0
//!         FLAGS              :
//!    ```
//! 3. Stop the driver using `sc stop snFilter`, should give the following output:
//!     ```ignore
//!    SERVICE_NAME: snFilter
//!         TYPE               : 2  FILE_SYSTEM_DRIVER
//!         STATE              : 1  STOPPED
//!         WIN32_EXIT_CODE    : 0  (0x0)
//!         SERVICE_EXIT_CODE  : 0  (0x0)
//!         CHECKPOINT         : 0x0
//!         WAIT_HINT          : 0x0
//!    ```
//! 4. Remove it by `sc delete snFilter`, should give the following output:
//!      ```ignore
//!    [SC] DeleteService SUCCESS
//!    ```
//!
//! You can also run `Fltmc.exe` to see the currently loaded drivers:
//!
//! ```ignore
//! Filter Name                     Num Instances    Altitude    Frame
//! ------------------------------  -------------  ------------  -----
//! bindflt                                 1       409800         0
//! snFilter                                4       378781         0   //! our minifilter driver
//! WdFilter                                5       328010         0
//! storqosflt                              0       244000         0
//! wcifs                                   0       189900         0
//! CldFlt                                  0       180451         0
//! FileCrypt                               0       141100         0
//! luafv                                   1       135000         0
//! npsvctrig                               1        46000         0
//! Wof                                     3        40700         0
//! FileInfo                                5        40500         0
//! ```
//!
//! ## Rust Application
//!
//! ### Building Rust App
//!
//! Simply use `cargo build --release` to build the application
//!
//! ### Running Rust App
//!
//! Use `cargo run --bin minifilter --release` to run the application
//!
//! The program starts to print the `IOMessage` which is defined like:
//!
//! ```ignore
//! #[repr(C)]
//! pub struct IOMessage {
//!     pub extension: [wchar_t; 12],
//!     pub file_id_vsn: c_ulonglong,
//!     pub file_id_id: [u8; 16],
//!     pub mem_sized_used: c_ulonglong,
//!     pub entropy: f64,
//!     pub pid: c_ulong,
//!     pub irp_op: c_uchar,
//!     pub is_entropy_calc: u8,
//!     pub file_change: c_uchar,
//!     pub file_location_info: c_uchar,
//!     pub filepathstr: String,
//!     pub gid: c_ulonglong,
//!     pub runtime_features: RuntimeFeatures,
//!     pub file_size: i64,
//! }
//! ```
//!
//! We end the process using `ctrl + c` in the example video:
//! ![video](https://!github.com/sn99/fsfilter-rs/readme_resources/example.gif)
//!
//! #### NOTE:
//!
//! - Might fail if not ran with administrative privileges
//! - You need to [load and start the driver]((https://!github.com/sn99/fsfilter-rs#loadingremoving-driver)) before running
//!   the program or else it will error out
//!
//! ## What and the How
//!
//! We basically share definition between the mini-filter and Rust using `#[repr(C)]`
//!
//! ![shared_def](https://!github.com/sn99/fsfilter-rs/readme_resources/shared_def.png)
//!
//! We use [channels](https://!doc.rust-lang.org/std/sync/mpsc/fn.channel.html) to process
//! all [IRPs](https://!docs.microsoft.com/en-us/windows-hardware/drivers/ifs/irps-are-different-from-fast-i-o).

pub mod driver_comm;
pub mod shared_def;
