# 维护规则

> 本文件按场景组织：每个场景给出触发条件、要检查/更新的文档、完成标准。类型格式见 [conventions.md](conventions.md)；地图与生命周期见 [README.md](README.md)。

## 场景索引

| 场景 | 何时用 | 主要更新 |
|---|---|---|
| [开始一次代码改动](#1-开始一次代码改动) | 目标明确，但还没有可执行 plan | design/analysis/decision/plan |
| [改代码/接口](#2-改代码接口) | 修改 `src/` | reference/design/decision/Q |
| [做一个决定](#3-做一个决定) | 多方案、难回退、真权衡 | decision/design/Q |
| [关闭 Q](#4-关闭-q) | 问题有答案或决定放弃 | questions/decision/design |
| [完成/放弃 plan](#5-完成放弃-plan) | 步骤完成或计划终止 | 活文档 + plan 归档 |
| [发现文档过期/被取代](#6-发现文档过期被取代) | `verified_at` 落后或有新文档 | status/banner/archive |
| [新增/移动/删除文档](#7-新增移动删除文档) | 文档集合变化 | README/front matter/链接 |
| [归档一轮迭代](#8-归档一轮迭代) | 一轮设计/实现结束 | archive/README + 结论提炼 |
| [定期审计](#9-定期审计) | 每月或每轮结束 | stale/覆盖矩阵/链接 |
| [提交前](#10-提交前) | 准备 commit | 链接/状态/同步检查 |

## 1. 开始一次代码改动

**触发**：有一个明确要改的行为或接口，但还没有可执行的 plan。

**要检查/更新**：

- [design](conventions.md#2-design)：问题/目标/非目标、模型、不变量、接口影响是否写清？
- [analysis](conventions.md#4-analysis)：是否还有关键未知？有就先调查，不要直接写 plan。
- [decision](conventions.md#5-decision)：是否需要在多个可行方案里拍板？难回退/反直觉/真权衡 → 写 ADR。
- [question](conventions.md#7-question)：把未决问题登记成 `Q-xx`，plan 的前置假设引用它。
- [plan](conventions.md#6-plan)：只有目标已定、接口影响已知、完成标准可验证时，才建 `plans/<id>/README.md`。

**完成标准**：plan 入口页能回答“为什么改、改成什么、分几步、接口怎么变、怎么验证”；每个未决问题有 `Q-ID`；没有占位步骤。

**常见错误**：目标未定就写步骤；把 analysis 当 design；为小改动建一堆空文档（小改动例外见 [plan 卡片](conventions.md#6-plan)）。

## 2. 改代码/接口

**触发**：修改 `src/`，尤其是 `interface.rs`、`current.rs`、`main_loop.rs`、`schedule/`。

**要检查/更新**：

- [reference](conventions.md#3-reference)：对应契约是否仍是 `current`？更新 `verified_at`；不能更新就标 `stale`。
- [design](conventions.md#2-design)：目标、不变量、接口影响变了吗？变了就更新。
- [decision](conventions.md#5-decision)：是否引入或推翻了难回退的选择？是就写/更新 ADR。
- [question](conventions.md#7-question)：是否解决或产生了 Q？关闭或新增。
- `plans/<id>/README.md`：如果是 plan 的一步，更新步骤状态；接口变了就更新接口变更表。
- `README.md` 1.2/1.3：只有新增/移动/删除文档时才需要。

**完成标准**：受影响的 reference 要么 `current`（`verified_at` 已更新），要么 `stale` 且有重验动作；接口变更表有旧→新→理由；相关 Q 已关闭或新增。

**常见错误**：只改代码不改 reference；把 `stale` 当“已处理”；接口变了不写理由。

## 3. 做一个决定

**触发**：在多个可行方案中选择，且难回退、反直觉或真有权衡。

**要检查/更新**：

- [decision](conventions.md#5-decision)：写 `decisions/NNNN-<slug>.md`，包含背景、决策、替代方案、后果、相关 Q/设计。
- [design](conventions.md#2-design)：把决定反映到目标、不变量、接口影响。
- [question](conventions.md#7-question)：相关 Q 标 `decided`，`详见` 指向 `D-xxx`。
- `plans/<id>/README.md`：如果决定影响执行，更新方案概述或接口变更。

**完成标准**：ADR 的替代方案和后果写清；design 已反映；Q 已关闭；受影响的 plan 已更新。

**常见错误**：把日常小决定写成 ADR；ADR 没有替代方案；决定后不改 design。

## 4. 关闭 Q

**触发**：一个问题有答案了，或决定放弃。

**要检查/更新**：

- [question](conventions.md#7-question)：标 `decided` / `dropped`；`详见` 指向 decision/design；**不删行**。
- [decision](conventions.md#5-decision)：如果是难回退的选择，写 ADR。
- [design](conventions.md#2-design)：如果是设计取舍，更新 design 的“未决问题/取舍”节。
- [reference](conventions.md#3-reference)：如果答案是现状事实，更新对应条目。
- `plans/<id>/README.md`：如果 Q 阻塞了 plan，解除 `blocked` 或更新前置假设。

**完成标准**：Q 行有 `decided` / `dropped` + `详见`；对应文档已更新；没有裸 TODO。

**常见错误**：删 Q 行；只改 Q 不写 ADR/design；`详见` 指向不存在的锚点。

## 5. 完成/放弃 plan

**触发**：plan 的所有步骤完成，或决定放弃。

**要检查/更新**：

- [reference](conventions.md#3-reference)：已按新行为更新，或明确标 `stale`。
- [design](conventions.md#2-design)：目标、不变量、接口影响已更新。
- [decision](conventions.md#5-decision)：过程中的难回退选择已写 ADR。
- [analysis](conventions.md#4-analysis)：结论已提取，标 `concluded`；被取代的标 `superseded`。
- [question](conventions.md#7-question)：相关 Q 已关闭。
- `plans/<id>/README.md`：完成时标 `done`；放弃时先做下面的“放弃时额外检查”，再标 `dropped`；然后归档到 `archive/`。

**放弃时额外检查**：

- 代码可编译、测试通过；未完成路径禁用或隔离，不要留在可达路径。
- 半迁移的接口要么完成，要么回滚到旧契约。
- `reference` 与代码实际行为一致；不一致就标 `stale` 并登记 Q。
- `plans/<id>/README.md` 记录：已改什么、已回滚什么、遗留什么、下一步。
- 以上完成后再标 `dropped` 并归档。

**完成标准**：

- `done`：活文档已反映变化、测试通过、Q 已关闭；plan 归档。
- `dropped`：代码/文档回到稳定状态，plan README 记录遗留与下一步；plan 归档。

**只合代码不算完成。**

**常见错误**：代码合入就归档 plan；活文档没更新；放弃的 plan 不写原因；半迁移状态直接归档。

## 6. 发现文档过期/被取代

**触发**：`verified_at` 落后、代码变了、出现新文档或新决定。

**要检查/更新**：

状态定义以 [conventions.md](conventions.md) 的类型卡片为准；下表只列“过期/取代/归档”场景的处理。

| 情况 | 处理 | 放哪 |
|---|---|---|
| reference 未跟上代码，主题仍存在 | 标 `stale`，加文首提示，安排重验 | 留在 `reference/` |
| reference 被新文档取代 | 标 `superseded` + `superseded_by`，更新引用 | `archive/` |
| reference 描述的对象已删除 | 删除，或归档并注明 | `archive/` 或删除 |
| design 被新设计取代 | `superseded` | 迭代结束后进 `archive/` |
| analysis 已得出结论 | 标 `concluded`，结论提取到 decision/design/reference | 留在 `analysis/` 作证据 |
| analysis 被新分析取代 | `superseded` + `superseded_by` | `archive/` |
| plan 完成/放弃 | `done` / `dropped` | `archive/` |
| decision 被新 ADR 取代 | `superseded` | 留在 `decisions/` |
| question 已决定/放弃 | `decided` / `dropped` | 留在 `questions.md` |

- **`stale` 留在原目录，不进 archive。** 它仍是该主题当前唯一的文档；移走会断链接、从导航消失，还会把“未重验”误当成“历史”。
- **`stale` 必须带一个动作**：重验（更新 `verified_at`，回到 `current`）、取代（写新文档/ADR，标 `superseded`）或删除（对象已不存在）。只标不办会变成 sediment。
- **可见性**：`reference/README.md` 的覆盖矩阵列出每篇的 `depth` / `status` / `verified_at`。文档内：
  - `stale`：`> **STALE**：最后验证于 <commit>；此后 <path> 有变更，使用前请重验。`
  - `superseded`：`> **SUPERSEDED**：见 <new-doc>；本文仅作历史。`
  - `current` 不加横幅。
- **归档不是“过期”的同义词**：只有 reference/design/analysis 的 `superseded`、已完成/放弃的计划，或整轮迭代才进 `archive/`；已取代的 decision 留在 `decisions/`。

## 7. 新增/移动/删除文档

**触发**：创建、移动、重命名或删除文档。

**要检查/更新**：

- 选类型：design / reference / analysis / decision / plan / question；格式见 [conventions.md](conventions.md) 对应卡片。
- 加 [front matter](conventions.md#81-front-matter)（导航、`questions.md`、`archive/` 除外）。
- 更新 [README.md](README.md) 的 1.2 登记；目标结构变化时更新 1.3。
- 如果它是高价值入口，再在 `AGENTS.md` 路由里加一行；**只指向已存在的文件**。
- 更新所有引用（入链/出链）；检查 [链接与锚点](conventions.md#83-文件链接与图)。
- 移动/删除时：先更新引用，再移动；`archive/` 只读，只修链接。

**完成标准**：新文档在 README 可见；front matter 合法；0 断链/0 坏锚点；旧位置没有残留引用。

**常见错误**：建了文档不登记；移动后不修引用；用空文件占位。

## 8. 归档一轮迭代

**触发**：一轮设计/实现结束，被新迭代取代。

**要检查/更新**：

- 先提炼结论到 `reference/`、`design/`、`decisions/`；不要直接搬走。
- `archive/<iteration>/`：放该轮文档、图、计划；`archive/README.md` 记录目标、结论、仍有效、被谁取代。
- 更新 `README.md` 1.3 的目标结构与引用；必要时更新 `conventions.md` 的 archive 说明。
- `archive/` 只读；只允许修正链接/路径、标注缺失资产。

**完成标准**：archive 里每条“已解决”都能在活文档中找到对应；活文档不把 archive 当现状引用。

**常见错误**：把 archive 当垃圾场；不提炼结论；归档后还改内容。

## 9. 定期审计

**触发**：每月，或每轮迭代结束。

**要检查/更新**：

- `reference/` 的 `stale` 文档和 `verified_at`；`reference/README.md` 覆盖矩阵的空格。
- `questions.md` 的长期 `open` / `leaning`，是否有阻塞。
- `plans/` 的 `blocked` 或长期 `active`，是否该 `dropped`。
- 全仓链接、锚点、裸 TODO。
- `archive/README.md` 是否完整。

**完成标准**：每个 `stale` 有动作；每个长期 Q 有下一步；0 断链/坏锚点/裸 TODO。

**常见错误**：只扫不处理；把审计当一次性任务。

## 10. 提交前

**触发**：准备 commit。

**要检查/更新**：

- 链接、锚点、裸 TODO、`verified_at`。
- 改代码 → reference/design/decision/Q/plan 是否同步。
- 新增/移动/删除文档 → README 1.2/1.3 是否更新。
- front matter 是否合法、状态是否准确。

**完成标准**：`git diff --check` 通过；0 断链/坏锚点；相关文档已更新或标 `stale`。

**常见错误**：只提交代码；把 `stale` 留到“以后”；提交信息不引用 Q/D。
