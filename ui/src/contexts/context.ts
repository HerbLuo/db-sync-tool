import { Dispatch, SetStateAction, useState } from "react";

export type Context<T> = {
  context: T, 
  setContext: (context: T) => void
};

export const nullContext = {
  context: null, 
  setContext: () => {},
};

export function useStateToUseContext<S>([context, setContext]: readonly [S, Dispatch<SetStateAction<S>>]): Context<S> {
  return {
    context,
    setContext,
  }
}

export function useContext<T>(def: T): Context<T> {
  const [context, setContext] = useState<T>(def);
  return {
    context,
    setContext,
  };
}
