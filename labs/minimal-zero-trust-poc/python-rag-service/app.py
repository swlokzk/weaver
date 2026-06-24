from collections import Counter
from math import sqrt
from flask import Flask, jsonify, request

app = Flask(__name__)

DOCUMENTS = [
    {"id": "okf-1", "text": "OKF uses structured knowledge facts for retrieval"},
    {"id": "okf-2", "text": "Zero Trust gateway validates identity and request integrity"},
    {"id": "okf-3", "text": "MCP server exposes tools to agents through routing"},
]


def tokenize(text: str) -> list[str]:
    return [w.lower() for w in text.split() if w.strip()]


def to_vector(text: str) -> Counter:
    return Counter(tokenize(text))


def cosine(a: Counter, b: Counter) -> float:
    dot = sum(a[k] * b[k] for k in a.keys() & b.keys())
    na = sqrt(sum(v * v for v in a.values()))
    nb = sqrt(sum(v * v for v in b.values()))
    if na == 0 or nb == 0:
        return 0.0
    return dot / (na * nb)


@app.get("/health")
def health():
    return jsonify({"ok": True})


@app.post("/search")
def search():
    data = request.get_json(force=True, silent=True) or {}
    query = str(data.get("query", "")).strip()
    top_k = int(data.get("top_k", 3))

    qv = to_vector(query)
    scored = []
    for doc in DOCUMENTS:
        score = cosine(qv, to_vector(doc["text"]))
        scored.append({"id": doc["id"], "text": doc["text"], "score": round(score, 4)})

    scored.sort(key=lambda x: x["score"], reverse=True)
    return jsonify({"query": query, "results": scored[:top_k]})


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8001)
