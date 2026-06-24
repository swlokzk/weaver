import express from "express";

const app = express();
app.use(express.json());

const RAG_SERVICE_URL = process.env.RAG_SERVICE_URL ?? "http://127.0.0.1:8001";

type JsonRpcRequest = {
  method: string;
  params?: any;
  id?: string | number;
};

app.post("/mcp", async (req, res) => {
  const rpc = req.body as JsonRpcRequest;

  if (rpc.method === "tools/list") {
    return res.json({
      id: rpc.id ?? null,
      result: {
        tools: [
          { name: "echo", description: "Echo text" },
          { name: "rag.search", description: "Search in RAG service" }
        ]
      }
    });
  }

  if (rpc.method === "tools/call") {
    const name = rpc.params?.name as string;
    const args = rpc.params?.arguments ?? {};

    if (name === "echo") {
      return res.json({ id: rpc.id ?? null, result: { text: String(args.text ?? "") } });
    }

    if (name === "rag.search") {
      const query = String(args.query ?? "");
      const topK = Number(args.top_k ?? 3);
      const response = await fetch(`${RAG_SERVICE_URL}/search`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ query, top_k: topK })
      });
      const result = await response.json();
      return res.json({ id: rpc.id ?? null, result });
    }

    return res.status(400).json({ id: rpc.id ?? null, error: "unknown tool" });
  }

  return res.status(400).json({ id: rpc.id ?? null, error: "unknown method" });
});

app.get("/health", (_req, res) => {
  res.json({ ok: true });
});

app.listen(8002, () => {
  console.log("mcp-server listening on :8002");
});
