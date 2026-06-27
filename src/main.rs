use std::cmp::Reverse;
use std::collections::HashMap;
use std::time;
use sysinfo::System;

struct ProcessNode {
    pid: u32,
    children: Vec<usize>,
}

struct ProcessTree {
    nodes: Vec<ProcessNode>,
    pid_map: HashMap<u32, usize>,
}

struct Monitor {
    processes: ProcessTree,
}

impl Monitor {
    pub fn build() -> Self {
        // Please note that we use "new_all" to ensure that all lists of
        // CPUs and processes are filled!
        let mut sys = System::new_all();

        // First we update all information of our `System` struct.
        sys.refresh_all();

        let mut pid_map: HashMap<u32, usize> = HashMap::new();
        let mut nodes: Vec<ProcessNode> = vec![];

        for (pid, process) in sys.processes() {
            // let is_root_process = process.parent().is_some_and(|x| x.as_u32() == 1);
            let process_node = ProcessNode {
                pid: pid.as_u32(),
                children: vec![],
            };

            nodes.push(process_node);
            pid_map.insert(pid.as_u32(), nodes.len() - 1);
        }

        let processes = ProcessTree { nodes, pid_map };

        Self { processes }
    }
}

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
