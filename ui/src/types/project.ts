import { SyncConfig } from "./sync-config";

export interface Project {
  name: string;
  def?: boolean;
  syncs: SyncConfig[];
}
