#![allow(unused_must_use)]
use fsfilter_rs::driver_comm;
use fsfilter_rs::shared_def::{CDriverMsgs, IOMessage};
use std::io::Write;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{io, thread};

fn main() {
    let driver = driver_comm::Driver::open_kernel_driver_com()
        .expect("Cannot open driver communication (is the mini-filter started?)");
    driver
        .driver_set_app_pid()
        .expect("Cannot set driver app pid");
    let mut vecnew: Vec<u8> = Vec::with_capacity(65536);

    let (tx_iomsgs, rx_iomsgs) = channel::<IOMessage>();

    thread::spawn(move || loop {
        if let Some(reply_irp) = driver.get_irp(&mut vecnew) {
            if reply_irp.num_ops > 0 {
                let drivermsgs = CDriverMsgs::new(&reply_irp);
                for drivermsg in drivermsgs {
                    let iomsg = IOMessage::from(&drivermsg);
                    if tx_iomsgs.send(iomsg).is_ok() {
                    } else {
                        panic!("Cannot send iomsg");
                    }
                }
            } else {
                // Don't use "continue" as jump statements are expensive "if reply_irp.num_ops > 0"
                // continue;
                thread::sleep(Duration::from_millis(2));
            }
        } else {
            panic!("Can't receive Driver Message?");
        }
    });

    {
        let mut lock = io::stdout().lock();
        loop {
            if let Ok(mut io_message) = rx_iomsgs.recv() {
                io_message.exepath();
                write!(lock, "{io_message:?}");
                lock.flush();
            }
        }
    }
}
