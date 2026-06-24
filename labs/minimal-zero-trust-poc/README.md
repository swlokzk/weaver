# Minimal TS + Rust + Python POC

此實驗用最小可運行實作驗證三層架構：

- Rust Gateway：Zero Trust 入口（mTLS 標識 + HMAC 請求簽名檢查）
- TypeScript MCP Server：基本工具路由（`echo` / `rag.search`）
- Python RAG Service：簡單語義檢索（bag-of-words + cosine）

> mTLS 在此以 `x-client-cert-fingerprint` header 模擬：適用於 TLS 在上游終止（如 envoy/nginx）並轉發已驗證 client cert 指紋的場景。

## Quick start

```bash
cd /home/runner/work/weaver/weaver/labs/minimal-zero-trust-poc
docker compose up --build -d
```

## Smoke tests

```bash
./scripts/test_rag.sh
./scripts/test_mcp.sh
./scripts/test_gateway.sh
```
