import { createContext, useState } from "react";
import { SyncConfig } from "../types/configuration";
import { Context, defaultContext } from "./context";

export type CurrentSyncConfigContextType = Context<SyncConfig | null>;

export const CurrentSyncConfigContext = createContext<CurrentSyncConfigContextType>(defaultContext);

export function useCurrentSyncConfigContextState(): Context<SyncConfig | null> {
  return useState<SyncConfig | null>(null);
}
