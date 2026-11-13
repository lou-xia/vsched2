# trap处理实现为事件源

对调度器的trap处理流程进行了修改，现在不止是外部中断，而是所有trap都会将trap信息放入trap处理队列后，进入调度器了。

（见修改后的流程图）

这样修改的好处在于进一步统一了trap优先级和任务优先级，避免优先级反转的情况。

## trap信息

与trap直接相关的信息（通常为trap相关的几个寄存器的取值）。实现为`TrapInfo` trait，由OS提供实现。

OS实现的接口：

- `TrapInfo::from_task`：从被trap的任务上下文中获取trap信息。
- `TrapInfo::handle`：处理该trap。

处理trap所需的信息除了`TrapInfo`以外，还包括（可选的）整个任务上下文（用任务指针代表）。处理与被trap任务相关的trap（系统调用、异常）时，任务指针为`Some`。处理与被trap任务无关的trap（中断）时，任务指针为`None`。无论是`TrapInfo::handle`的参数，还是trap队列的元素，均包含了`TrapInfo`和`Option<*const ()>`（任务指针）两项。

## trap队列

per-cpu实现，每个CPU上包含一条存储`(TrapInfo, Option<*const ()>)`的队列和一个trap处理任务。

该队列实现为事件源。当当前核心选取下一任务时，会从包括trap队列和就绪队列的多条事件源中选择优先级最高的任务并运行。作为事件源，trap队列提供的任务即为trap处理任务。

## trap处理任务

定义了trap处理函数`trap_handler`，作为trap处理任务中运行的函数。

OS需实现`TrapInfo::new_handler`接口，以`trap_handler`为运行的函数创建一个任务，并将其指针返回。之后由调度器管理该任务。

## 新的trap处理流程

进入trap_entry后，分配预保存栈，之后根据trap是否与被trap任务相关，设置被trap任务的状态为阻塞或就绪。之后，根据任务创建`TrapInfo`，将`TrapInfo`和任务（相关的情况）或仅`TrapInfo`（无关的情况）加入trap队列。

之后进入调度器，由调度器选择最高优先级的任务（通常为trap处理任务）并运行。

在trap处理任务中，调用`TrapInfo::handle`处理trap。每处理完一个trap，如果有任务与该trap关联，则唤醒该任务。
