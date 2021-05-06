export interface ClientAddr {
  hostname: string;
  username: string;
  db: string,
  port: number,
  password: string,
}

export interface SyncConfig {
  name?: string;
  mode: "drop-create";
  tables: "*" | string[];
  from: string | ClientAddr;
  to: string | ClientAddr;
  buffer_size?: number;
  skip_sync_if_table_not_exist?: boolean;
}
