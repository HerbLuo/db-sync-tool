import { SyncConfig } from "../types/sync-config";

export interface Project {
  name: string;
  def?: boolean;
  syncs: SyncConfig[];
}

type Projects = Project[];

const defProject: Project = {
  name: "默认",
  syncs: [],
};

const KEY = "projects";

export async function getAll(): Promise<Projects | null> {
  const projects = localStorage.getItem(KEY);
  return projects ? JSON.parse(projects) as Projects : [defProject];
}

export async function saveAll(projects: Projects): Promise<void> {
  localStorage.setItem(KEY, JSON.stringify(projects));
}
