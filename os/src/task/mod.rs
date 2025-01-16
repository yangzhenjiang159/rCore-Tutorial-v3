//! 任务管理实现
//!
//! 关于任务管理的一切，比如启动和切换任务
//! 在这里实现。
//!
//! ['TaskManager']的单个全局实例称为TASK_MANAGER控件
//! 操作系统中的所有任务。
//!
//! 在'switch. S'中看到'__switch'ASM函数时要小心。围绕此函数的控制流
//! 可能不是你所期望的。
mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

///任务管理器，管理所有任务。
///
///在“TaskManager”上实现的函数处理所有任务状态转换
///和任务上下文切换。为方便起见，您可以在它周围找到包装器
///在模块级别。
///
///大多数“TaskManager”隐藏在字段“内部”后面，以延迟
///借用检查到运行时。您可以在
///“TaskManager”上的现有函数。
pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}
/// Inner of Task Manager
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
}

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            }
        }
    };
}

impl TaskManager {
    /// 运行任务列表中的第一个任务。
    ///
    /// 通常，任务列表中的第一个任务是空闲任务（我们以后称之为零进程）。
    /// 但是在 ch3 中，我们是静态加载应用程序的，所以第一个任务是一个真正的应用程序。
    fn run_first_task(&self) -> !{
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

    ///把"Running" 改为 "Ready"状态
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    /// 找到下一个任务,变更为就绪态,并返回appid
    /// 这种情况下,我们只返回任务列表里第一个"Ready"的任务
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1 .. current + self.num_app + 1)
            .map(|index| index % self.num_app)
            .find(|i| inner.tasks[*i].task_status == TaskStatus::Ready)
    }

    /// 找到"Ready"任务并切换当前"Running"的任务, 如果所有任务中没有"Ready"的任务时,退出
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);

            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }
}

/// run first task
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}


/// rust next task
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// exit current task
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// suspend current task, then run next task
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// exit current task,  then run next task
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
