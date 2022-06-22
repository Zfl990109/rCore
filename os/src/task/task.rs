//! Types related to task management

use super::TaskContext;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info: TaskInfo,
    pub last_start_time: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}


pub const MAX_SYSCALL_NUM : usize = 100;		// 支持的 syscall 的最大数量
#[derive(Copy, Clone)]
pub struct SyscallInfo {
	pub id: usize,					// syscall_id
	pub times: usize,				// 使用的系统调用次数
}
impl SyscallInfo {
	/// Creates a new [`SyscallInfo`].
    pub fn new() -> Self{
		SyscallInfo {
			id: 0,
			times: 0,
		}
	}
}
    
#[derive(Copy, Clone)]
/// TaskInfo
pub struct TaskInfo {
    /// 任务 id
	pub id: usize, 		
    /// 任务状态
	pub status: TaskStatus,
    /// 任务使用的系统调用信息	  
	pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    /// 任务运行的总时间 
	pub time: usize,  
}
impl TaskInfo {
	/// Creates a new [`TaskInfo`].
    pub fn new() -> Self{
		TaskInfo {
			id: 0,
			status: TaskStatus::Ready,
			call: [SyscallInfo::new(); MAX_SYSCALL_NUM],
			time: 0,
		}
	}
}