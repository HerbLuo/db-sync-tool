import { createContext, useState } from "react";
import { SyncConfig } from "../types/sync-config";
import { Context, defaultContext } from "./context";

export type CurrentSyncConfigContextType = Context<SyncConfig | null>;

export const CurrentSyncConfigContext = createContext<CurrentSyncConfigContextType>(defaultContext);

export function useCurrentSyncConfigContextState(): Context<SyncConfig | null> {
  return useState<SyncConfig | null>(null);
}
