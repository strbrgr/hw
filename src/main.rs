use std::cmp::Reverse;
use std::time;
use sysinfo::System;

fn main() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    loop {
        let mut processes: Vec<_> = sys.processes().iter().collect();
        processes.sort_by_key(|a| Reverse(a.1.memory()));
        for (pid, process) in processes.iter().take(10) {
            println!(
                "[{pid}] {:?} {:?} MiB, parent: {:?}",
                process.name(),
                process.memory() / 1024 / 1024,
                process.parent()
            );
        }

        std::thread::sleep(time::Duration::from_secs(1));
    }
}
