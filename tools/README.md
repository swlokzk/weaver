# Tools

## 目的 / Purpose

Agent 可直接呼叫或組合的工具。

Tools that agents or humans can directly use or combine.

## 特徵 / Characteristics

- 給 agent 或人直接使用 / For direct use by agents or humans
- 有清楚輸入/輸出 / Clear inputs/outputs
- 能被自動化呼叫 / Can be called automatically
- 是能力接口 / Capability interfaces

## 內容範例 / Content Examples

- CLI tools
- MCP tools
- Repo transformers
- Extractors
- Utilities for agent usage

## 命名原則 / Naming Principles

應該像 / Should be like:
- `fetch_repo_tree`
- `select_high_value_files`
- `build_skill_context`
- `generate_skill_card`

避免 / Avoid:
- `do_magic`
- `run_pipeline`
- `process_data`
