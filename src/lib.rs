//! 可移植的统一任务调度模块

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

mod interface;
mod schedule_loop;
mod stack;
