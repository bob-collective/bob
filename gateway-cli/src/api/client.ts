import type {
  RouteInfo,
  GatewayQuoteParams,
  GatewayQuote,
  GatewayCreateOrderResponse,
  GatewayOrderInfo,
  MaxSpendable,
  RegisterTxRequest,
  RegisterTxResponse,
} from "./types.js";
import { verbose } from "../util/progress.js";

export class GatewayApiError extends Error {
  constructor(
    public readonly statusCode: number,
    public readonly body: unknown,
    public readonly endpoint: string,
  ) {
    const msg = typeof body === 'object' && body !== null && 'message' in body
      ? (body as any).message
      : JSON.stringify(body);
    super(`Gateway API error (${statusCode}) on ${endpoint}: ${msg}`);
    this.name = "GatewayApiError";
  }
}

export class GatewayApiClient {
  constructor(private baseUrl: string) {}

  async getRoutes(): Promise<RouteInfo[]> {
    return this.get("/v1/get-routes");
  }

  async getQuote(params: GatewayQuoteParams): Promise<GatewayQuote> {
    const searchParams = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined) searchParams.set(key, String(value));
    }
    return this.get(`/v1/get-quote?${searchParams.toString()}`);
  }

  async createOrder(quote: GatewayQuote): Promise<GatewayCreateOrderResponse> {
    return this.post("/v1/create-order", quote);
  }

  async registerTx(req: RegisterTxRequest): Promise<RegisterTxResponse> {
    return this.patch("/v1/register-tx", req);
  }

  async getOrder(id: string): Promise<GatewayOrderInfo> {
    return this.get(`/v1/get-order/${encodeURIComponent(id)}`);
  }

  async getOrders(userAddress: string): Promise<GatewayOrderInfo[]> {
    return this.get(`/v1/get-orders/${encodeURIComponent(userAddress)}`);
  }

  async getMaxSpendable(address: string): Promise<MaxSpendable> {
    return this.get(`/v1/get-max-spendable/${encodeURIComponent(address)}`);
  }

  private async parseErrorBody(res: Response): Promise<unknown> {
    try { return await res.json(); } catch { return await res.text(); }
  }

  private async get<T>(path: string): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    verbose(`GET ${url}`);
    const res = await fetch(url, { method: "GET" });
    verbose(`← ${res.status} ${res.statusText}`);
    if (!res.ok) {
      const body = await this.parseErrorBody(res);
      throw new GatewayApiError(res.status, body, path);
    }
    return res.json() as Promise<T>;
  }

  private async post<T>(path: string, body: unknown): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    verbose(`POST ${url}`);
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    verbose(`← ${res.status} ${res.statusText}`);
    if (!res.ok) {
      const errBody = await this.parseErrorBody(res);
      throw new GatewayApiError(res.status, errBody, path);
    }
    return res.json() as Promise<T>;
  }

  private async patch<T>(path: string, body: unknown): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    verbose(`PATCH ${url}`);
    const res = await fetch(url, {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    verbose(`← ${res.status} ${res.statusText}`);
    if (!res.ok) {
      const errBody = await this.parseErrorBody(res);
      throw new GatewayApiError(res.status, errBody, path);
    }
    return res.json() as Promise<T>;
  }
}
