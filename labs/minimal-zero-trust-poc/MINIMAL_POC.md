# Minimal TS + Rust + Python POC — System & Solution Architecture

最小可運行參考實作：TypeScript (MCP) + Rust (Zero Trust Gateway) + Python (RAG)

Elevator (一句話)
- 本 PoC 演示如何把 Zero Trust 邊緣（Rust）、MCP 工具面（TypeScript）與語義檢索（Python RAG）組合成一個簡潔、可啟動的參考架構，用於驗證跨語言協作與關鍵模式（OKF → MCP → RAG）的可行性。
- This PoC demonstrates a minimal, runnable stack combining a Rust-based Zero Trust gateway, a TypeScript MCP server, and a Python RAG service to validate cross-language patterns for OKF → MCP → RAG.

---

## 目錄 / Contents
- Overview / 概述
- System Architecture / 系統架構（Component responsibilities, ASCII diagram）
- API Surface / 介面說明（MCP & RAG endpoints）
- Quickstart / 快速啟動
- Smoke tests / 煙霧測試（curl 範例）
- Security notes（PoC 限制 & 生產化強化）
- Design decisions & Roadmap / 設計要點與後續規劃
- Files & Locations / 關鍵檔案位置
- Contact / 聯絡

---

## Overview / 概述
本實驗目標：
- 驗證「Zero Trust ingress (Rust) → MCP orchestration (TS) → RAG enhanced retrieval (Python)」的最小可行流程。
- 支持 OKF-style knowledge 查詢通路（OKF 作為知識輸入格式可與此 PoC 結合）。
- 提供 Docker Compose 可啟動參考與基本 smoke tests。

This project validates the minimal flow: Zero Trust ingress (Rust) → MCP server (TS) → semantic retrieval (Python), suitable for prototyping OKF→MCP→RAG integration.

---

## System Architecture / 系統架構

責任分配（Component responsibilities）
- Rust Gateway (ports: 8000)
  - Zero Trust ingress responsibilities: validate client identity (simulated client-cert fingerprint header), verify request integrity (HMAC signature over method+path+body).
  - Forward validated requests to MCP Server.
  - PoC note: mTLS is *simulated* by upstream-provided header (x-client-cert-fingerprint).
- TypeScript MCP Server (ports: 8002)
  - Exposes MCP-like JSON-RPC surface: `tools/list`, `tools/call` (includes `echo`, `rag.search`).
  - Routes `rag.search` to the RAG service.
- Python RAG Service (ports: 8001)
  - Minimal semantic search (in-memory bag-of-words + cosine).
  - Endpoints: `/health`, `/search`.

ASCII diagram
```
[Client] ---(mTLS upstream)--> [Rust Gateway (Zero Trust)] --HTTP--> [TS MCP Server (tools/list, tools/call)]
                                                       └--> [Python RAG Service (search)] 
```

Sequence (request flow)
1. Client creates payload and HMAC-signature header `x-signature` (over method+path+body) and includes `x-client-cert-fingerprint`.
2. Rust Gateway verifies fingerprint & HMAC, then forwards to MCP Server URL (`/mcp`).
3. MCP Server handles `tools/call` and for `rag.search` forwards query to Python RAG service.
4. RAG returns ranked results; MCP returns to Gateway; Gateway forwards response to client.

---

## API Surface / 介面說明

Rust Gateway (forwarding)
- POST /gateway/mcp
  - Headers:
    - x-client-cert-fingerprint: <trusted_fp>
    - x-signature: <hmac_sha256_hex(method+"\n"+path+"\n"+body)>
  - Body: JSON RPC-style payload forwarded to MCP

TypeScript MCP Server
- POST /mcp
  - body: {
      "method": "tools/call" | "tools/list",
      "params": { ... }
    }
- Supported `tools/call` actions:
  - `echo`: returns back the params
  - `rag.search`: params { "query": string, "top_k": int }

Python RAG Service
- GET /health
  - returns { "ok": true }
- POST /search
  - body: { "query": "text", "top_k": 3 }
  - returns: { "query": "...", "results": [ {id, text, score}, ... ] }

---

## Quickstart / 快速啟動

Prerequisites: Docker & Docker Compose installed.

啟動：
```bash
cd labs/minimal-zero-trust-poc
docker compose up --build -d
```

停止 / 清理：
```bash
docker compose down
```

---

## Smoke Tests / 煙霧測試（示例 curl）

1) 檢查 RAG 健康：
```bash
curl -s http://localhost:8001/health | jq
# expect {"ok": true}
```

2) 直接查 RAG：
```bash
curl -s -X POST http://localhost:8001/search \
  -H "Content-Type: application/json" \
  -d '{"query":"OKF knowledge"}' | jq
```

3) 直接呼叫 MCP（繞過 gateway，直接到 MCP）：
```bash
curl -s -X POST http://localhost:8002/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "method": "tools/call",
    "params": { "name": "rag.search", "arguments": {"query":"Zero Trust", "top_k":2} }
  }' | jq
```

4) 透過 Gateway（帶簽名）呼叫 MCP:
- 範例產生 HMAC（示意，bash）：
```bash
BODY='{"method":"tools/call","params":{"name":"rag.search","arguments":{"query":"mcp","top_k":2}}}'
SIGNING_SECRET=demo-signing-secret
SIGN_INPUT=$'POST\n/gateway/mcp\n'"$BODY"
SIG=$(printf '%s' "$SIGN_INPUT" | openssl dgst -sha256 -hmac "$SIGNING_SECRET" -binary | xxd -p -c 256)
curl -s -X POST http://localhost:8000/gateway/mcp \
  -H "Content-Type: application/json" \
  -H "x-client-cert-fingerprint: demo-client-fp" \
  -H "x-signature: $SIG" \
  -d "$BODY" | jq
```

注意：PoC 假設 upstream 代理（例如 Envoy/Nginx）已處理 mTLS，並將 client cert 指紋放入 header。範例中使用 `demo-client-fp` 與 `demo-signing-secret`。

---

## Security notes（PoC 限制 & 生產化要點）
PoC 簡化與模擬：
- mTLS is simulated by `x-client-cert-fingerprint` header — production must terminate real TLS and validate cert chains.
- HMAC shared secret is a PoC simplification — production should use per-tenant keys/PKI with rotation, KMS integration.
- No persistent audit chain (append-only log) in PoC — production should write audit to immutable store (e.g., PostgreSQL + signed chain or append-only ledger).
- RAG uses in-memory bag-of-words — for real semantics use embeddings + vector DB (Faiss/Milvus/Pinecone) + secure storage and access control.
- No tenant isolation, authz policy, rate-limiting or quotas in PoC; add those for multi-tenant deployments.

Production hardening checklist:
- Use real mTLS (Rust Gateway + cert manager)
- Migrate HMAC → per-tenant signed JWTs or mutual TLS + request signing
- Secrets management (HashiCorp Vault / AWS KMS)
- Persistent logging + tamper-evident audit trail
- Deploy each component in containers orchestrated by Kubernetes, add HPA and proper resource requests/limits
- Use vector DB and embedder service for RAG

---

## Design decisions & Roadmap / 設計要點與後續規劃
Why this split?
- Keep the ingress Gatekeeper (security-critical) in a memory-safe, high-performance language (Rust).
- Keep protocol orchestration and light routing in TypeScript (fast iteration, small runtime).
- Keep heavy ML / vector operations in Python (rich ML ecosystem).

Roadmap (next steps):
1. Replace BOW RAG with embeddings → Faiss/Milvus + SentenceTransformer.
2. Add persistent OKF store in `weaver/knowledge` and automatic sync from `swlokzk/learning`.
3. Harden gateway: real mTLS termination + per-tenant auth and KMS.
4. Add observability: Prometheus + Grafana + distributed tracing (Jaeger).
5. K8s manifests & Helm charts for production deployment.

---

## Files & Locations / 關鍵檔案位置
- labs/minimal-zero-trust-poc/README.md (this file)
- labs/minimal-zero-trust-poc/docker-compose.yml
- labs/minimal-zero-trust-poc/python-rag-service/app.py
- labs/minimal-zero-trust-poc/ts-mcp-server/ (TypeScript MCP server)
- labs/minimal-zero-trust-poc/rust-gateway/ (Rust gateway)
- labs/minimal-zero-trust-poc/scripts/ (smoke test helpers)

---

## Minimal System & Solution Architect summary (1–2 lines)
- /weaver is a lightweight reference runtime that composes a Rust Zero‑Trust gateway, a TypeScript MCP server, and a Python RAG service to validate OKF→MCP→RAG integration and zero‑trust ingress patterns.
- /weaver 結合 Rust（Zero Trust）、TypeScript（MCP）與 Python（RAG），用以驗證並展示 OKF 知識查詢、MCP 工具暴露與受控語義檢索的端到端設計。

---

## Contact / 聯絡
若需要我把 README 直接提交為 PR（替換原檔），或只要我產生一個精簡摘要段落放回原 README，請回覆確認，我可以直接替你提交修改。