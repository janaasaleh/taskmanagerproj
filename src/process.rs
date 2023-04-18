use sysinfo::ProcessExt;
use std::time::{Duration, Instant};

#[derive(PartialEq, Clone)]
pub struct Process {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
    pub time: Instant,
    pub parent: Option<sysinfo::Pid> //not sure of this
    //pub parent: Option<pid>

}

impl Process {
    pub fn new(process: &sysinfo::Process) -> Process {
        Process {
            pid: process.pid(),
            name: process.name().to_string(),
            cpu: process.cpu_usage(),
            mem: process.memory(),
            time: Instant::now(),
            parent: process.parent()
        }
    }

    pub fn format(&self) -> Vec<String> {
        let parent_string = match self.parent {
            Some(pid) => pid.to_string(),
            None => String::from("N/A")
        };

        let time_string = self.time.elapsed().as_secs_f32().to_string();
    

        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            time_string,
            parent_string
        ]
    }
}
