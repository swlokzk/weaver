# 🦾🏗️ Weaver

這是一個自用的 Agent 功能模組管理 Monorepo，採用 pnpm workspace 架構。核心目的在於**將 LLM 接入、MCP 協議、原子工具（Tools）與複合工作流（Skills）進行扁平化、集中化管理**，作為個人其他終端應用的底層依賴庫。

---

## 📂 扁平化目錄與職責邊界

```text
weaver/
├── llm/          # 大模型適配：統一封裝 Provider API、Token 計算與 Stream 處理
├── mcp/          # 協議層：處理標準 JSON-RPC 封裝與 Context 注入
├── tools/        # 原子工具庫：無狀態、純函數、不含 LLM 推理的底層 I/O 操作
└── skills/       # 複合技能包：有狀態、包含 System Prompt、串聯多個 Tools 的工作流

```

---

## 📐 依賴防護原則 (Dependency Boundary)

為了確保各個本地模組能被乾淨地解耦複用，本倉庫嚴格遵守以下**單向依賴拓撲**：

```text
[skills]  ─── 組合/呼叫 ───>  [tools]
   │                             │
   └──────── 共同底層 ──────────> [llm & mcp]

```

* **tools（原子工具）**：保持絕對純粹。只處理基礎輸入與輸出，**內部不允許引入 llm**。工具本身不需要、也不應該知道是哪個模型在調用它。
* **skills（複合技能）**：業務邏輯與流程的聚合點。所有的 Prompt 拼接、Few-shot 範例引導、以及多步驟 Tool 的條件分支串聯（Workflow）皆收攏於此。
* **mcp（標準接口）**：將內部的 tools 或 skills 包裝為符合 Model Context Protocol 規範的標準功能，供本地其他桌面客戶端或網關直接調用。
