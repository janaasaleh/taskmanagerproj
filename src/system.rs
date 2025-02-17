use sysinfo::{SystemExt, ProcessorExt, ProcessExt};
use std::time::{SystemTime,UNIX_EPOCH};
use crate::process::Process;

   pub static mut SYSTEM_START_TIME: u64 = 0;

unsafe fn updatesystemstarttime() {
    SYSTEM_START_TIME = (SystemTime::now()).duration_since(UNIX_EPOCH).unwrap().as_secs();
}
pub struct System {
    sysinfo: sysinfo::System,
    pub cpu_usage_history: Vec<u64>,
    pub cpu_current_usage: u64,
    pub cpu_num_cores: usize,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub mem_usage_history: Vec<u64>,
    pub cpu_core_usages: Vec<u16>,
    pub processes: Vec<Process>,
}

impl System {
    pub fn new(initial_size: u16) -> System {
        let sysinfo = sysinfo::System::new();

        let history_width = initial_size / 2;

        // Overall CPU usage
        let cpu_usage_history = vec![0; history_width as usize];
        let cpu_num_cores: usize = sysinfo.get_processor_list().len() - 1;

        // Memory usage
        let mem_total = sysinfo.get_total_memory();
        let mem_usage_history = vec![0; history_width as usize];

        unsafe {updatesystemstarttime();}

        System {
            sysinfo,
            cpu_usage_history,
            cpu_current_usage: 0,
            cpu_num_cores,
            mem_total,
            mem_free: 0,
            mem_used: 0,
            mem_usage_history,
            cpu_core_usages: vec![],
            processes: vec![]
        }
    }

    pub fn update(&mut self) -> System {
        self.sysinfo.refresh_all();

        // Overall CPU usage
        self.cpu_current_usage = (self.sysinfo.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
        self.cpu_usage_history.push(self.cpu_current_usage);
        self.cpu_usage_history.remove(0);

        // Memory usage
        self.mem_used = self.sysinfo.get_used_memory();
        self.mem_free = self.sysinfo.get_free_memory();
        self.mem_usage_history.push(self.mem_used);
        self.mem_usage_history.remove(0);

        // CPU core usage
        self.cpu_core_usages = self.sysinfo.get_processor_list()
            .iter()
            .skip(1)
            .map(|p| (p.get_cpu_usage() * 100.0).round() as u16)
            .collect();

        // Processes
        self.processes = self.sysinfo.get_process_list()
            .iter()
            .map(|(_, process)|
                Process::new(process)
            )
            .collect();

        System {
            sysinfo: sysinfo::System::new(),
            cpu_usage_history: self.cpu_usage_history.clone(),
            mem_usage_history: self.mem_usage_history.clone(),
            cpu_core_usages: self.cpu_core_usages.clone(),
            processes: self.processes.clone(),
            ..*self
        }
    }

    pub fn kill_process(&mut self, pid: i32) {
        if let Some(process) = self.sysinfo.get_process(pid) {
            process.kill(sysinfo::Signal::Kill);
        }
    }

    // pub fn stop_process(&mut self, pid: i32) {
    //     if let Some(process) = self.sysinfo.get_process(pid) {
    //         process.stop(sysinfo::Signal::Stop);
    //     }
    // }
}
