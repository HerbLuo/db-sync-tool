import { ClientAddr } from "../types/configuration";

export function formatAddr(addr: string | ClientAddr): ClientAddr {
  if (typeof addr === "string") {
    throw new Error("暂不支持str");
  }
  return addr;
}
