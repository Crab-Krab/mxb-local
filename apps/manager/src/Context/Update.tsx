import { createContext, useContext } from "react";
import type { Update } from "@tauri-apps/plugin-updater";
import { toast } from "sonner";
type UpdateContextValue = {
  available: Update | null; installing: boolean; progress: number | null;
  check: (opts?: { silent?: boolean }) => Promise<void>;
  install: () => Promise<void>; dismiss: () => void;
};
const UpdateContext = createContext<UpdateContextValue | null>(null);
export function UpdateProvider({ children }: { children: React.ReactNode }) {
  return <UpdateContext.Provider value={{available:null, installing:false, progress:null,
    check:async (opts) => { if (!opts?.silent) toast.info("MXB Local uses manually reviewed builds. Automatic updates are disabled."); },
    install:async () => { toast.info("Use a reviewed MXB Local build to update."); }, dismiss:() => {}}}>{children}</UpdateContext.Provider>;
}
export function useUpdate() {
  const value = useContext(UpdateContext);
  if (!value) throw new Error("useUpdate must be used within UpdateProvider");
  return value;
}
