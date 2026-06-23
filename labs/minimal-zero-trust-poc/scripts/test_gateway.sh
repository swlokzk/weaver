#!/usr/bin/env bash
set -euo pipefail

BODY='{"id":3,"method":"tools/call","params":{"name":"echo","arguments":{"text":"gateway"}}}'
SIG=$(python - <<'PY'
import hmac, hashlib
secret = b"demo-signing-secret"
body = '{"id":3,"method":"tools/call","params":{"name":"echo","arguments":{"text":"gateway"}}}'
msg = f"POST\n/gateway/mcp\n{body}".encode()
print(hmac.new(secret, msg, hashlib.sha256).hexdigest())
PY
)

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST http://127.0.0.1:8000/gateway/mcp \
  -H 'content-type: application/json' \
  -H "x-client-cert-fingerprint: demo-client-fp" \
  -H "x-signature: bad-signature" \
  -d "$BODY")
[ "$HTTP_CODE" = "401" ]

curl -sS -X POST http://127.0.0.1:8000/gateway/mcp \
  -H 'content-type: application/json' \
  -H "x-client-cert-fingerprint: demo-client-fp" \
  -H "x-signature: ${SIG}" \
  -d "$BODY" | grep 'gateway' >/dev/null

echo "Rust gateway smoke test passed"
