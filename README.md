# vsched2

vsched2 是一个 Rust `no_std` 的可移植任务调度模块。它通过 vDSO 把同一份调度代码映射到内核态和用户态：每个进程有自己的用户态调度器，所有内核任务共用一个内核调度器。

## 它解决什么问题

- 调度代码只写一份，内核态/用户态复用，减少两套调度器的行为差异。
- 任务不只包括线程和协程，也包括 trap 处理等执行流；调度器统一按优先级选择。
- 任务切换、trap 入口、栈管理和阻塞唤醒都在模块边界内；操作系统负责提供任务、地址空间、栈和 trap 信息的适配接口。

更完整的概念见 [共享调度器模块设计文档](doc/draft/新版设计/共享调度器模块设计文档.md)。

## 当前状态

**文档与重构阶段，设计未定稿，不是稳定版本。**

- `doc/draft/` 是迁移起点，混合“已实现/目标/待定”，不能当现状。
- 目标文档结构（`design/`、`reference/`、`plans/` 等）多数仍是 planned，见 [doc/README.md 1.3](doc/README.md#13-目标结构)。
- 当前阶段与下一步见 [doc/文档整理计划.md](doc/文档整理计划.md)。

## 快速开始

本仓库是 vDSO 模块，不是独立可执行程序。构建/集成通过同级的 `vdso_builder`：

```bash
cd ../vdso_builder
CPU_NUM=4 cargo run
```

详见 [编译命令](doc/编译命令.md)。仓库内没有自动测试；验证依赖外部 OS/测例或场景走查。

## 代码入口

- `src/schedule/`：调度器、就绪队列、事件源、trap 等待队列。
- `src/main_loop.rs`：调度循环、trap/线程入口、任务切换路径。
- `src/interface.rs`：OS 需要实现的接口。
- `src/current.rs`：VVAR、当前任务、当前地址空间等全局状态。
- `src/block_and_wake/`、`src/stack/`、`src/arch/`：阻塞唤醒、栈、架构相关。

代码与文档的对应关系见 [doc/README.md](doc/README.md) 的 1.5。

## 文档与协作

- [doc/README.md](doc/README.md)：文档地图、生命周期、阅读路径。
- [AGENTS.md](AGENTS.md)：AI agent 的路由与硬规则。
- [doc/conventions.md](doc/conventions.md)：文档类型、状态机、格式。
- [doc/maintenance.md](doc/maintenance.md)：改完代码/文档后要更新什么。

第一次读建议从 [doc/README.md](doc/README.md) 的“现在（迁移前）”路径开始。
