import { useState } from "react";

export function useRefresh(): [number, () => void] {
  const [signal, refresh] = useState(0);
  return [signal, () => refresh(signal + 1)];
}