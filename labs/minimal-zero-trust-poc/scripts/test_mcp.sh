#!/usr/bin/env bash
set -euo pipefail

curl -sS http://127.0.0.1:8002/health | grep '"ok":true' >/dev/null
curl -sS -X POST http://127.0.0.1:8002/mcp \
  -H 'content-type: application/json' \
  -d '{"id":1,"method":"tools/list"}' | grep 'rag.search' >/dev/null
curl -sS -X POST http://127.0.0.1:8002/mcp \
  -H 'content-type: application/json' \
  -d '{"id":2,"method":"tools/call","params":{"name":"echo","arguments":{"text":"hello"}}}' | grep 'hello' >/dev/null

echo "MCP server smoke test passed"
