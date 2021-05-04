import { useCallback, useState } from "react";

export function useSwitch(def = false) {
  const [open, setOpen] = useState(def);
  const toggleState = useCallback(() => {
    setOpen(o => !o);
  }, []);
  return [open, toggleState] as const;
}
