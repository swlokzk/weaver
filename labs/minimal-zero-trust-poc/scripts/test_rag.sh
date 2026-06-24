#!/usr/bin/env bash
set -euo pipefail

curl -sS http://127.0.0.1:8001/health | grep '"ok":true' >/dev/null
curl -sS -X POST http://127.0.0.1:8001/search \
  -H 'content-type: application/json' \
  -d '{"query":"zero trust integrity","top_k":1}' | grep 'okf-2' >/dev/null

echo "RAG service smoke test passed"
