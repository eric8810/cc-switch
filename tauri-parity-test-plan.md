# Tauri Baseline / Parity Test Plan

## 阶段边界

当前分支只做 `src-tauri` 的测试验收，不做正式迁移。

- 允许新增测试、fixture、文档和极薄的测试辅助代码
- 不改 `src-tauri` 运行时命令归属
- 不新增 `bridge` 层
- 不把 Tauri command 改成转调 `cc-switch-core`

换句话说，这一阶段的目标是先把“现在的真实行为”冻结下来，作为后续 parity 和正式迁移的基线。

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

- 在 baseline 冻结完成后，用同一组 fixture 对比 legacy 与 core 结果

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
- `omo`
  - 本地配置文件读取
- `stream_check`
  - config save/get
- `webdav`
  - WebDAV settings save
- `global_proxy`
  - 本地代理端口扫描

## Case List

### P0: 现状冻结

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
- [x] OMO local file read
- [x] stream-check config save / get
- [x] WebDAV settings save
- [x] global proxy local scan

### P1: 前端 API 契约冻结

- [x] provider switch invoke contract
- [x] settings save invoke contract
- [x] webdav save invoke contract
- [x] workspace daily memory invoke contract
- [x] session messages invoke contract
- [x] skills import invoke contract
- [x] global proxy invoke contract

### P2: 后续 parity 前置清单

- [ ] failover queue / auto-failover 的 stateful parity harness
- [ ] proxy start/stop/takeover 的 shell smoke
- [ ] usage request detail / pricing CRUD baseline
- [ ] universal provider baseline
- [ ] OpenClaw agents/tools/default-model baseline
- [ ] OMO current-provider stateful baseline

## 执行顺序

1. 先维持这条 baseline 分支只做测试验收
2. baseline 稳定后，再单开 parity 分支
3. parity 跑通后，才开始逐域正式迁移

## 验收标准

满足下面条件，才允许进入正式迁移：

- `src-tauri` 现状已有稳定 baseline
- 前端 API 到 command 的映射已有 smoke 保护
- 测试不依赖 `bridge` 或已迁移的 command 路由
- 文档能明确区分 baseline、parity、正式迁移三个阶段
