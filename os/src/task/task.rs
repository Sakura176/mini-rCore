// os/src/task/task.rs
use super::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    // 当前任务状态
    pub task_status: TaskStatus,
    // 任务上下文
    pub task_cx: TaskContext,
}
