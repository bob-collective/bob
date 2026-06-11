const UPSTREAM = "https://gateway-api-mainnet.gobob.xyz/v2/get-routes";

export default async function handler(req: any, res: any) {
  try {
    const upstream = await fetch(UPSTREAM);
    if (!upstream.ok) {
      res.status(502).json({ error: "Failed to fetch routes" });
      return;
    }
    const data = await upstream.json();
    res.status(200).json(data);
  } catch {
    res.status(502).json({ error: "Failed to fetch routes" });
  }
}
