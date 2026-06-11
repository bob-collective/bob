const UPSTREAM = "https://gateway-api-mainnet.gobob.xyz/v2/get-routes";

export default async function handler(req: any, res: any) {
  if (req.method !== "GET") {
    res.status(405).json({ error: "Method not allowed" });
    return;
  }
  try {
    const upstream = await fetch(UPSTREAM);
    if (!upstream.ok) {
      res.status(502).json({ error: "Failed to fetch routes" });
      return;
    }
    const data = await upstream.json();
    res.status(200).json(data);
  } catch (err) {
    console.error("get-routes proxy error:", err);
    res.status(502).json({ error: "Failed to fetch routes" });
  }
}
