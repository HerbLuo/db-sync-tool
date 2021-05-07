export type Context<T> = readonly [T, (context: T) => void];

export const defaultContext = [null, () => {}] as const;
