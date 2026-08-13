use crate::interface::{SMPVirtImpl, Stack, StackVirtImpl, CPU_NUM, SMP, STACK_POOL_SIZE};
use heapless::index_map::FnvIndexMap;
use vdso_helper::log::warn;

// /// 对栈进行封装
// ///
// /// 用来储存栈基址，后续可以加入新的内容。通过调用 alloc 和 dealloc 接口进行栈的分配和回收
// ///
// /// ### 注意
// ///
// /// 这里需要保证之后对 Task 进行封装时，线程的栈使用的是 StackWapper，否则在栈池中的`switch_to_thread_stack`会错误的调用 Drop
// ///
// /// ### TODO
// ///
// /// 考虑使用 manually drop 来解决上述问题
// #[derive(Debug)]
// pub struct StackWapper {
//     /// 栈基址
//     pub base: usize,
//     /// 是否是初始栈（不能被释放）
//     pub is_init: bool,
// }

// impl StackWapper {
//     /// 分配一个新的栈
//     pub fn new() -> Self {
//         let base = StackVirtImpl::alloc() as usize;
//         Self {
//             base,
//             is_init: false,
//         }
//     }

//     /// 从一个已有的栈中获取一个新实例
//     pub fn from_raw(base: usize) -> Self {
//         Self {
//             base,
//             is_init: false,
//         }
//     }

//     /// 根据栈底创建新的初始栈实例
//     pub fn from_raw_init(base: usize) -> Self {
//         Self {
//             base,
//             is_init: true,
//         }
//     }

//     /// 回收栈
//     pub fn dealloc(self) {
//         assert!(!self.is_init);
//         StackVirtImpl::dealloc(self.base as *mut ());
//     }
// }

// // impl Drop for StackWapper {
// //     /// 回收栈
// //     fn drop(&mut self) {
// //         StackVirtImpl::dealloc(self.base as *mut ());
// //     }
// // }

// impl Default for StackWapper {
//     fn default() -> Self {
//         Self {
//             base: 0,
//             is_init: false,
//         }
//     }
// }

// impl From<usize> for StackWapper {
//     fn from(base: usize) -> Self {
//         Self {
//             base,
//             is_init: false,
//         }
//     }
// }

// impl From<StackWapper> for usize {
//     fn from(stack: StackWapper) -> Self {
//         stack.base
//     }
// }

/// 栈池管理器
///
/// 管理栈的分配和回收，提供栈切换的接口
#[derive(Debug)]
pub struct StackHandler {
    /// 空栈的集合
    pub(crate) free_stacks:
        [FnvIndexMap<usize, &'static mut StackVirtImpl, STACK_POOL_SIZE>; CPU_NUM],
    /// 当前使用的栈
    pub(crate) current_stack: [Option<&'static mut StackVirtImpl>; CPU_NUM],
    /// 放入sscratch等寄存器中，供中断入口使用的栈
    pub(crate) trap_stack: [Option<&'static mut StackVirtImpl>; CPU_NUM],
}

impl StackHandler {
    // /// 创建一个新的栈管理器
    // pub(crate) fn new() -> Self {
    //     let mut stacks = Vec::new();
    //     for _ in 0..STACK_POOL_SIZE - CPU_NUM {
    //         stacks
    //             .push(unsafe { StackVirtImpl::from_mut(StackVirtImpl::alloc()) })
    //             .expect("failed to create new stack");
    //     }
    //     Self {
    //         free_stacks: stacks,
    //         current_stack: [Some(unsafe { StackVirtImpl::from_mut(StackVirtImpl::alloc()) });
    //             CPU_NUM],
    //     }
    // }

    pub(crate) fn alloc_stack(&mut self, cpu_id: usize) -> &'static mut StackVirtImpl {
        // 这里因为持有的是可变引用，所以两次获取应该没有问题？
        if let Some((&addr, _)) = self.free_stacks[cpu_id].iter().next() {
            self.free_stacks[cpu_id].remove(&addr).unwrap()
        } else {
            unsafe { StackVirtImpl::from_mut(StackVirtImpl::alloc()) }
        }
    }

    pub(crate) fn dealloc_stack(&mut self, stack: &'static mut StackVirtImpl, cpu_id: usize) {
        let addr = stack as *mut StackVirtImpl as usize;
        self.free_stacks[cpu_id].remove(&addr);
        match self.free_stacks[cpu_id].insert(addr, stack) {
            Err((_, stack)) => stack.dealloc(),
            Ok(_) => {}
        }
    }

    pub(crate) fn set_current_stack(
        &mut self,
        stack: &'static mut StackVirtImpl,
        cpu_id: usize,
    ) -> Option<&'static mut StackVirtImpl> {
        let stack_ptr = stack as *mut _ as usize;
        let stack_base = stack.base() as usize;
        // 检查当前栈是否在其它核心上被使用
        for i in (0..cpu_id).chain(cpu_id + 1..CPU_NUM) {
            if let Some(current_stack) = &self.current_stack[i] {
                let current_stack_ptr = (*current_stack) as *const _ as usize;
                let current_base = current_stack.base() as usize;
                if current_stack_ptr == stack_ptr && current_base == stack_base {
                    panic!(
                        "Error: Stack {:#x}(base {:#x}) is already in use by CPU {}",
                        stack_ptr, stack_base, i
                    );
                }
            }
        }
        let old_stack = self.current_stack[cpu_id].replace(stack);
        if let Some(old_stack) = old_stack {
            let old_stack_ptr = old_stack as *mut _ as usize;
            let old_base = old_stack.base() as usize;
            // warn!(
            //     "set current stack: {:#x}(base {:#x}) -> {:#x}(base {:#x})",
            //     old_stack_ptr, old_base, stack_ptr, stack_base
            // );
            Some(old_stack)
        } else {
            // warn!(
            //     "set current stack: None -> {:#x}(base {:#x})",
            //     stack_ptr, stack_base
            // );
            None
        }
    }

    pub(crate) fn take_current_stack(&mut self, cpu_id: usize) -> &'static mut StackVirtImpl {
        let stack = self.current_stack[cpu_id]
            .take()
            .expect("Error: Failed to take current stack");
        let stack_ptr = stack as *mut _ as usize;
        let stack_base = stack.base() as usize;
        // warn!(
        //     "set current stack: {:#x}(base {:#x}) -> None)",
        //     stack_ptr, stack_base
        // );
        stack
    }

    /// 为当前核心分配trap栈并写入`trap_stack`变量中。
    ///
    /// 返回分配的栈基址，后续需要将其写入对应寄存器（如`sscratch`）中。
    ///
    /// 如果当前地址空间有处理trap的需求，则需在初始化时调用该函数。
    ///
    /// 否则，无需调用。
    pub(crate) fn alloc_trap_stack(&mut self, cpu_id: usize) -> *mut () {
        // for i in 0..CPU_NUM {
        //     let stack = self.alloc_stack();
        //     let old = self.trap_stack[i].replace(stack);
        //     assert!(old.is_none());
        // }
        let stack = self.alloc_stack(cpu_id);
        let base = stack.base();
        // warn!("alloc trap stack: {:#x}", base as usize);
        let old = self.trap_stack[cpu_id].replace(stack);
        assert!(old.is_none());
        base
    }

    /// 将传入的栈写入`trap_stack`变量中。传入的栈需要已经放入`sscratch`等寄存器中，供中断入口使用。
    ///
    /// 该函数返回`trap_stack`中原有的栈，因为在中断入口中调用，因此返回的栈就是`sscratch`的旧值，也就是当前正在使用的栈。
    ///
    /// 之后，需要将返回的栈写入`current_stack`中，以维护栈状态。
    pub(crate) fn set_trap_stack(
        &mut self,
        stack: &'static mut StackVirtImpl,
        cpu_id: usize,
    ) -> Option<&'static mut StackVirtImpl> {
        self.trap_stack[cpu_id].replace(stack)
    }

    /// self切换到空栈，返回空栈的栈底。
    ///
    /// 对应黄色框部分
    ///
    /// 参数：
    /// - `stack_status`: 代表当前栈的状态，0为空栈，1为非空栈。
    pub(crate) fn get_empty_stack(&mut self, _stack_type: usize) -> usize {
        let cpu_id = SMPVirtImpl::cpu_id();
        if self.current_stack[cpu_id].is_none() {
            // 非空栈，需要切到空栈
            let empty_stack = self.alloc_stack(cpu_id);
            // warn!(
            //     "get_empty_stack: switch to stack {:#x}",
            //     empty_stack as *mut _ as usize
            // );
            let old_stack = self.set_current_stack(empty_stack, cpu_id);
            // info!("get_empty_stack");
            assert!(old_stack.is_none());
        }
        self.current_stack[cpu_id].as_ref().unwrap().base() as usize
    }

    /// 切换到线程栈
    ///
    /// 对应蓝色框部分
    ///
    /// 参数：
    /// - `thread_stack`: 代表线程栈，如果为 None，则不设置当前栈，否则将当前栈设置为 thread_stack。
    /// - `stack_status`: 代表当前栈的状态，0为空栈，1为非空栈。
    pub(crate) fn get_thread_stack(
        &mut self,
        thread_stack: Option<&'static mut StackVirtImpl>,
        _stack_type: usize,
    ) {
        let old_stack = {
            if let Some(stack) = thread_stack {
                // warn!(
                //     "get_thread_stack: switch to stack {:#x}",
                //     stack as *mut _ as usize
                // );
                let stack = self.set_current_stack(stack, SMPVirtImpl::cpu_id());
                // info!("get_thread_stack, thread_stack = Some");
                stack
            } else {
                let stack = self.current_stack[SMPVirtImpl::cpu_id()].take();
                // info!("take current_stack: {:#x?}", stack);
                // info!("get_thread_stack, thread_stack = None");
                stack
            }
        };
        if let Some(old_stack) = old_stack {
            let cpu_id = SMPVirtImpl::cpu_id();
            self.dealloc_stack(old_stack, cpu_id);
        }
    }
}

impl Default for StackHandler {
    fn default() -> Self {
        // Self::new() 这样合适还是现在这样全0之后再初始化合适？
        Self {
            free_stacks: [const { FnvIndexMap::new() }; CPU_NUM],
            current_stack: [const { None }; CPU_NUM],
            trap_stack: [const { None }; CPU_NUM],
        }
    }
}
