mod context;
mod switch;
mod task;

use lazy_static::*;
use log::debug;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

use crate::loader::{get_num_app, init_app_cx};
use crate::sbi::shutdown;
use crate::{config::MAX_APP_NUM, sync::UPSafeCell};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        // 获取应用数量
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner { tasks, current_task: 0 })
            },
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }
    
    // 寻找下一个就绪的任务，采用环形遍历搜索策略
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // Rust迭代器，左闭右开，
        // 采用环形遍历搜索，遍历长度为num_app
        // map取模映射，避免越界
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn run_next_task(&self) {
        debug!("run_next_task");
        if let Some(next) = self.find_next_task() {
            debug!("success find next task");
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr =
                &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr =
                &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            debug!("after switch!");
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }
    
    // 标记当前任务为挂起状态
    fn mark_current_suspended(&self) {
        debug!("mark_current_suspended");
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }
    
    // 标记当前任务为退出状态
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
}
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}
pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}
pub fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}
pub fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
