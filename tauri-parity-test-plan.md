# Tauri Baseline / Parity Test Plan

## 阶段边界

当前分支只做 `src-tauri` 的测试验收，不做正式迁移。

- 允许新增测试、fixture、文档和极薄的测试辅助代码
- 不改 `src-tauri` 运行时命令归属
- 不新增 `bridge` 层
- 不把 Tauri command 改成转调 `cc-switch-core`

换句话说，这一阶段的目标是先把“现在的真实行为”尽量完整地冻结下来，作为后续 `legacy vs core parity` 和正式迁移的基线。

这里要特别强调：

- baseline 不是“先做一部分就算完成”
- baseline 的目标是尽可能完整覆盖当前 `src-tauri` 的真实行为
- 只有 baseline 覆盖到足够完整，后面拿同一批 case 去压 `core` 才有意义

## 测试分层

### Layer A: src-tauri baseline

目标：

- 直接验证当前 `src-tauri` 命令层和服务层行为
- 固定 live file、副作用、数据库读写、返回结构的现状

当前位置：

- 既有域测试：`src-tauri/tests/*.rs`
- 新增 baseline 套件：`src-tauri/tests/tauri_baseline.rs`

### Layer B: frontend API contract smoke

目标：

- 冻结 `src/lib/api/*` 到 Tauri command 名称和 payload 结构的映射
- 避免后续迁移时前端 quietly drift

当前位置：

- `tests/api/TauriApiContracts.test.ts`

### Layer C: legacy vs core parity

目标：

- 在 baseline 完整覆盖后，用同一组 fixture 对比 legacy 与 core 结果

当前状态：

- 本阶段不启动

### Layer D: 正式切换

目标：

- 分域把 Tauri command 接到 core
- 每切一块就跑 baseline + parity + smoke

当前状态：

- 本阶段不启动

## 当前 baseline 覆盖

### 已有测试

- `src-tauri/tests/app_type_parse.rs`
  - AppType 解析和错误分支
- `src-tauri/tests/app_config_load.rs`
  - 多应用配置加载
- `src-tauri/tests/provider_commands.rs`
  - provider switch / import default config
- `src-tauri/tests/provider_service.rs`
  - provider service 主链
- `src-tauri/tests/mcp_commands.rs`
  - MCP 导入和 app toggle
- `src-tauri/tests/proxy_commands.rs`
  - cost multiplier / pricing model source
- `src-tauri/tests/import_export_sync.rs`
  - export / import / sync current providers live
- `src-tauri/tests/deeplink_import.rs`
  - deeplink parse / import

### 新增 baseline 套件

- `settings`
  - save/get
  - terminal / visible apps 持久化
- `prompt`
  - 从 live file 导入 prompt
  - 当前 prompt file 内容快照
- `session`
  - Codex session 扫描
  - session messages 读取
- `usage`
  - 秒级时间戳窗口兼容
  - request detail
  - pricing CRUD
- `workspace`
  - daily memory 写入 / 列表 / 读取
- `env`
  - shell env 冲突删除与恢复
- `plugin`
  - Claude plugin 应用
  - onboarding skip
- `skill`
  - unmanaged scan
  - import
  - toggle
  - zip install
- `openclaw`
  - env round-trip
  - default model / agents defaults / tools round-trip
- `omo`
  - 本地配置文件读取
  - current-provider disable / clear state
- `stream_check`
  - config save/get
- `webdav`
  - WebDAV settings save
- `global_proxy`
  - 本地代理端口扫描
- `failover`
  - queue round-trip
  - auto-failover stateful enable / disable
- `proxy runtime`
  - start / takeover / stop / restore smoke
- `universal provider`
  - round-trip / sync / delete

## Case List

### Phase 1：baseline 完整覆盖

这一步的完成标准不是“有一套 baseline 框架”，而是“当前重要命令面和状态链路都已有稳定验收测试”。

#### 已完成

- [x] provider switch / import default config
- [x] provider service baseline
- [x] mcp import / toggle
- [x] proxy pricing settings
- [x] import / export / sync
- [x] deeplink parse / import
- [x] settings save / get
- [x] prompt import from live file
- [x] session scan / messages
- [x] usage summary over historical seconds timestamps
- [x] workspace daily memory round-trip
- [x] env conflict delete / restore
- [x] Claude plugin apply / onboarding skip
- [x] skill unmanaged import / toggle / zip install
- [x] OpenClaw env round-trip
- [x] OpenClaw default model / agents defaults / tools round-trip
- [x] OMO local file read
- [x] OMO current-provider disable / clear state
- [x] stream-check config save / get
- [x] WebDAV settings save
- [x] global proxy local scan
- [x] failover queue / auto-failover 的 stateful baseline
- [x] proxy start / stop / takeover / restore 的 shell smoke baseline
- [x] usage request detail / pricing CRUD baseline
- [x] universal provider baseline
- [x] OpenClaw agents/tools/default-model baseline
- [x] OMO current-provider stateful baseline

### Phase 1.5：前端 API 契约冻结

- [x] provider switch invoke contract
- [x] settings save invoke contract
- [x] webdav save invoke contract
- [x] workspace daily memory invoke contract
- [x] session messages invoke contract
- [x] skills import invoke contract
- [x] global proxy invoke contract
- [x] failover queue / auto-failover invoke contract
- [x] proxy runtime invoke contract
- [x] usage detail / pricing invoke contract
- [x] universal provider invoke contract
- [x] OpenClaw config invoke contract
- [x] OMO invoke contract

### Phase 2：legacy vs core parity

这一步只在 Phase 1 真正补齐后启动。

- [ ] 把 Phase 1 的 case 逐项复用到 legacy vs core parity
- [ ] 对比返回值、错误语义、live file、副作用、数据库最终状态
- [ ] 对 stateful runtime 场景补单独的 parity harness

## 执行顺序

1. 先维持这条 baseline 分支只做测试验收
2. 继续把 baseline 补到接近完整覆盖，不提前进入迁移
3. baseline 足够完整后，再单开 parity 分支
4. parity 跑通后，才开始逐域正式迁移

## 验收标准

满足下面条件，才允许进入 `legacy vs core parity`：

- `src-tauri` 现状已有稳定 baseline
- 关键状态链路也已有 baseline，而不只是静态命令快照
- 前端 API 到 command 的映射已有 smoke 保护
- 测试不依赖 `bridge` 或已迁移的 command 路由
- 文档能明确区分 baseline、parity、正式迁移三个阶段

满足下面条件，才允许进入正式迁移：

- baseline 已完整或接近完整覆盖当前 `src-tauri` 主链
- `legacy vs core parity` 已跑通
- `core` 在相同 case 下的返回值、副作用和状态与 legacy 足够对齐
