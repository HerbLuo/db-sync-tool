import { ClientAddr } from "../types/app-db";
import { logger } from "./logger";

export function formatAddr(addr: string | ClientAddr): ClientAddr {
  if (typeof addr === "string") {
    logger.warn("暂不支持str");
  }
  return {} as any;
}
