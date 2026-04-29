//! 就绪队列，实现为一个事件源
//!
//! todo：预存优先级，且考虑优先级的同步问题

use core::sync::atomic::{AtomicIsize, Ordering};

use heapless::{mpmc::Queue, BinaryHeap, Deque};
use spin::mutex::Mutex;

use crate::{
    interface::{Task, TaskVirtImpl, HIGHEST_PRIORITY, LOWEST_PRIORITY, READY_QUEUE_SIZE},
    schedule::event_source::EventSorce,
};

const PRIORITY_LEVELS: usize = (LOWEST_PRIORITY - HIGHEST_PRIORITY) as usize + 1;

pub(crate) struct ReadyQueue {
    /// index = priority - HIGHEST_PRIORITY
    queues: [Mutex<Deque<&'static TaskVirtImpl, READY_QUEUE_SIZE>>; PRIORITY_LEVELS],
    // hightest_prio: AtomicIsize,
}

impl ReadyQueue {
    pub(crate) const fn new() -> Self {
        Self {
            queues: [const { Mutex::new(Deque::new()) }; PRIORITY_LEVELS],
            // hightest_prio: AtomicIsize::new(LOWEST_PRIORITY + 1),
        }
    }

    pub(crate) fn push(&self, task: &'static TaskVirtImpl) -> Result<(), &'static TaskVirtImpl> {
        // 放入队列 -> 更新优先级
        let prio = task.priority();
        self.queues[(prio - HIGHEST_PRIORITY) as usize]
            .lock()
            .push_back(task)?;

        // loop {
        //     let current_prio = self.hightest_prio.load(Ordering::Acquire);
        //     if prio < current_prio {
        //         if self
        //             .hightest_prio
        //             .compare_exchange(current_prio, prio, Ordering::AcqRel, Ordering::Acquire)
        //             .is_ok()
        //         {
        //             break;
        //         }
        //     } else {
        //         break;
        //     }
        // }
        Ok(())
    }
}

impl EventSorce for ReadyQueue {
    fn hightest_priority(&self, _cpu_id: usize) -> isize {
        // self.hightest_prio.load(Ordering::Acquire)
        let mut prio = HIGHEST_PRIORITY;
        while prio <= LOWEST_PRIORITY {
            let queue = &self.queues[(prio - HIGHEST_PRIORITY) as usize];
            if !queue.lock().is_empty() {
                break;
            }
            prio += 1;
        }
        prio
    }

    fn take_task(&self, _cpu_id: usize) -> (*const (), isize) {
        // let original_prio = self.hightest_prio.load(Ordering::Acquire);
        // let mut prio = original_prio;
        let mut prio = HIGHEST_PRIORITY;
        while prio <= LOWEST_PRIORITY {
            let queue = &self.queues[(prio - HIGHEST_PRIORITY) as usize];
            if let Some(task) = queue.lock().pop_front() {
                // // 更新优先级
                let mut next_prio = prio;
                while next_prio <= LOWEST_PRIORITY {
                    let next_queue = &self.queues[(next_prio - HIGHEST_PRIORITY) as usize];
                    if !next_queue.lock().is_empty() {
                        break;
                    }
                    next_prio += 1;
                }
                // self.hightest_prio.store(next_prio, Ordering::Release);
                return (task.to_ptr(), next_prio);
            }
            prio += 1;
        }
        (core::ptr::null(), prio)
    }
}
