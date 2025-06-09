mod context;
mod switch;
mod task;

use lazy_static::*;
use log::{debug, trace};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

use crate::loader::{get_num_app, init_app_cx};
use crate::sbi::shutdown;
use crate::timer::get_time_ms;
use crate::{config::MAX_APP_NUM, sync::UPSafeCell};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
    stop_watch: usize,
}

impl TaskManagerInner {
    fn refresh_stop_watch(&mut self) -> usize {
        let start_time = self.stop_watch;
        self.stop_watch = get_time_ms();
        self.stop_watch - start_time
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        // 获取应用数量
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            user_time: 0,
            kernel_time: 0,
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate().take(num_app) {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner { tasks, current_task: 0, stop_watch: get_time_ms() })
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
        // 开始记录时间
        inner.refresh_stop_watch();
        drop(inner);
        // 初始化用户态上下文
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
        // debug!("run_next_task");
        if let Some(next) = self.find_next_task() {
            // debug!("success find next task");
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            trace!("task {} run!!!", current);
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
            // debug!("after switch!");
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }

    // 标记当前任务为挂起状态
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        trace!("task {} suspended!", current);
        // 统计内核时间
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    // 标记当前任务为退出状态
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // 统计内核时间并输出
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        debug!(
            "task {} exited, kernel_time: {} ms, user_time: {} ms",
            current,
            inner.tasks[current].kernel_time,
            inner.tasks[current].user_time
        );
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn user_time_start(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
    }

    fn user_time_end(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].user_time += inner.refresh_stop_watch();
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
pub fn user_time_start() {
    TASK_MANAGER.user_time_start()
}

pub fn user_time_end() {
    TASK_MANAGER.user_time_end()
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
