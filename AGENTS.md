# AGENTS.md

> 项目级 AI 使用说明。先读 [doc/README.md](doc/README.md)；本文只放路由和硬规则，不重复设计。

## 项目一句话

vsched2 是 Rust `no_std` 的可移植任务调度模块：通过 vDSO 在内核态和用户态复用调度代码与数据；每个进程一个调度器，内核共用一个调度器。当前处于文档与重构阶段，设计未定稿。

## 先读

1. [doc/README.md](doc/README.md)：地图、生命周期、阅读路径、代码→文档归属。
2. [GLOSSARY.md](GLOSSARY.md)：术语与代码符号。
3. [doc/conventions.md](doc/conventions.md)：文档类型、状态机、front matter、目录 README。
4. [doc/maintenance.md](doc/maintenance.md)：改完代码/文档后要更新什么。
5. [doc/文档整理计划.md](doc/文档整理计划.md)：当前阶段与下一步。
6. [plans/0003-scheduler-refactor](doc/plans/0003-scheduler-refactor/README.md)：三阶段重构计划与当前步骤。

## 路由

| 任务 | 先看 | 代码 |
|---|---|---|
| 调度循环、trap、栈、切换路径 | [设计文档](doc/draft/新版设计/共享调度器模块设计文档.md)、[代码实现设计](doc/draft/新版设计/共享调度器模块代码实现设计.md) | `src/main_loop.rs`、`src/stack/` |
| 用户态任务被中断后的归属/上下文 | [user-interrupt-context](doc/design/user-interrupt-context.md) | `src/main_loop.rs`、`src/interface.rs` |
| 接口、Task trait、数据结构 | [数据结构修改](doc/draft/新版设计/数据结构修改.md)、[代码实现设计](doc/draft/新版设计/共享调度器模块代码实现设计.md) 的接口/数据结构节 | `src/interface.rs`、`src/api.rs` |
| 新版设计计划改什么 | [new-design-planned-changes](doc/analysis/new-design-planned-changes.md) | — |
| VVAR、当前任务、可重入 | [可重入设计文档](doc/draft/可重入分析/调度器可重入设计文档.md)、[二维分析](doc/draft/可重入分析/二维分析.xlsx) | `src/current.rs` |
| 阻塞、唤醒、Waker | [代码实现设计](doc/draft/新版设计/共享调度器模块代码实现设计.md) 的阻塞/接口节 | `src/block_and_wake/` |
| 事件源、优先级、调度器 | [设计文档](doc/draft/新版设计/共享调度器模块设计文档.md) 的“调度器/事件源”节 | `src/schedule/` |
| 写/改文档 | [conventions.md](doc/conventions.md)、[maintenance.md](doc/maintenance.md) | — |
| 做计划/当前步骤 | [0003-scheduler-refactor](doc/plans/0003-scheduler-refactor/README.md)、[plan 卡](doc/conventions.md#6-plan)、[文档整理计划.md](doc/文档整理计划.md) | — |
| 查历史 | [doc/archive/](doc/archive/)（只读） | — |
| 构建 | [编译命令](doc/编译命令.md)、`build.rs`、`Cargo.toml` | — |

## 硬规则

- 先读 `doc/README.md`，不要凭文件名猜结构。
- `doc/draft/` 是迁移起点，混合“已实现/目标/待定”，不能当现状；`doc/archive/` 只读，不能当现状。
- 文档分层：`reference` = 现状，`design` = 目标，`decisions` = 选择，`analysis` = 证据，`plan` = 这次改动。不要把目标写成现状。
- 改代码改变契约 → 同一 commit 更新 `reference`；不能更新就标 `stale`，不要留下“看起来是 current”的旧文档。
- 未决问题用 `Q-ID`，写进 `doc/questions.md`；不写裸 TODO。
- 接口/函数签名改动：先写进 plan 的接口变更表（旧→新→理由），并按全局规则征得同意。
- 计划只在“目标已定、接口影响已知、完成标准可验证”时才写；不要写占位步骤。
- plan 完成的标准是活文档已更新、测试通过、Q 已关闭，不是代码合入。
- 文档用中文正文、代码原名（含拼写，如 `hightest_priority`）；不抄大段代码，给 `src/...:line`。
- 链接用仓库相对路径；标题锚点用 slug；不要写 `../26.x` 或外部“必读”链接。

## 验证

- 构建：`cd ../vdso_builder && CPU_NUM=4 cargo run`，详见 [编译命令](doc/编译命令.md)。
- 仓库内没有自动测试；改动后至少说明验证方式（编译、场景走查、外部测例）。
- 提交前检查链接、锚点、裸 TODO、`verified_at`。

## 待建文档

[doc/README.md 1.3](doc/README.md#13-目标结构) 列了目标结构；`reference/`、`design/` 多数仍是 `planned`；当前计划见 `plans/0003-scheduler-refactor`。不要链接不存在的文件；创建时按 [conventions.md](doc/conventions.md) 加 front matter，并在 `doc/README.md` 的 1.2 登记。
