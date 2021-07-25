export interface ClientAddr {
  id: string;
  hostname: string;
  username: string;
  db: string;
  port: number;
  password: string;
}

export interface SyncConfig {
  id: string;
  name?: string;
  mode: "drop-create";
  tables: "*" | string[];
  from: string | ClientAddr;
  to: string | ClientAddr;
  buffer_size?: number;
  skip_sync_if_table_not_exist?: boolean;
}

export interface AppDb {
  databaseAddresses: ClientAddr[];
  current: SyncConfig;
  syncConfigs: SyncConfig[];
}
