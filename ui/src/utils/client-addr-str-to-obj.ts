import { ClientAddr } from "../types/sync-config";

export function formatAddr(addr: string | ClientAddr): ClientAddr {
  if (typeof addr === "string") {
    throw new Error("暂不支持str");
  }
  return addr;
}
