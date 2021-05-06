import { useEffect, useMemo, useState } from "react";

export function usePromise<T>(promise: Promise<T>) {
  const [state, setState] = useState<T>();
  // eslint-disable-next-line
  const memoedPromise = useMemo(() => promise, []);
  useEffect(() => {
    memoedPromise.then(s => {
      setState(s);
    });
  }, [memoedPromise]);
  return [state, setState] as const;
}
