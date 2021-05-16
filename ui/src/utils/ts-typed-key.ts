export type TypedKey<O, T> = Exclude<{
  [K in keyof O]: T extends O[K] ? K : never;
}[keyof O], undefined>;
