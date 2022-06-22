use crate::syscall::{SyscallInfo, MAX_SYSCALL_NUM};

#[derive(Copy, Clone)]
pub struct TaskInfo {
	pub id: usize,
	pub status: TaskStatus,
	pub call: [SyscallInfo; MAX_SYSCALL_NUM],
	pub time: usize,
}

impl TaskInfo {
	pub fn new() -> Self{
		TaskInfo {
			id: 0,
			status: TaskStatus::Ready,
			call: [SyscallInfo::new(); MAX_SYSCALL_NUM],
			time: 0,
		}
	}

	pub fn print_task_info(&self) {
		println!("id:{}  status:{}  time:{}", 
				self.id, self.status.to_string(), self.time);
		let mut i = 0;
		while self.call[i].times != 0 {
			println!("syscall:{} has been used {} times", self.call[i].id, self.call[i].times);
			i += 1;
		}
	}
}


#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskStatus {
    pub fn to_string(&self) -> &str {
        match self {
            TaskStatus::UnInit => "UnInit",
            TaskStatus::Ready => "Ready",
            TaskStatus::Running => "Running",
            TaskStatus::Exited => "Exited",
        }
    }
}