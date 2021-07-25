import { createContext, useMemo } from "react";
import { AppDb, SyncConfig } from "../types/app-db";

type NullableAppDb = AppDb | null;
interface ContextType {
  appDb: NullableAppDb;
  setAppDb(appDb: NullableAppDb): void;
  setCurrent(current: SyncConfig): void;
  setSyncConfigs(syncConfigs: SyncConfig[]): void;
}

export const AppDbContext = createContext<ContextType>({
  appDb: null, 
  setAppDb: () => {}, 
  setCurrent: () => {}, 
  setSyncConfigs: () => {}
});

export function useAppDbContext([appDb, setAppDb]: readonly [NullableAppDb, (s: NullableAppDb) => void]): ContextType {
  return useMemo(() => ({
    appDb,
    setAppDb,
    setCurrent: (current) => setAppDb(appDb ? {...appDb, current} : null),
    setSyncConfigs: (syncConfigs) => setAppDb(appDb ? {...appDb, syncConfigs} : null),
  }), [appDb, setAppDb]);
}
