import { open as openUrl } from "@tauri-apps/plugin-shell";

/** Link the upstream credit separately from this fork's product name. */
const ABOUT_URL = "https://github.com/Frostn1/mxb-app";

/**
 * The wordmark names MXB Local and retains an explicit credit to its source project.
 */
export default function Brand({ name }: { name: string }) {
  return (
    <div data-tauri-drag-region className="flex select-none items-baseline gap-2">
      <span className="font-cond text-[16px] font-extrabold leading-none tracking-[-0.06em]">
        {name}
      </span>
      {/* Upstream credit links to the original source; this is an independent build. */}
      <button
        type="button"
        onClick={() => void openUrl(ABOUT_URL)}
        title={ABOUT_URL}
        className="group cursor-default font-cond text-[11px] font-medium leading-none tracking-[-0.02em] text-faint transition-colors hover:text-muted-foreground"
      >
        based on{" "}
        <span className="font-semibold text-muted-foreground decoration-muted-foreground/50 underline-offset-[3px] group-hover:underline">
          Frostn1/mxb-app
        </span>
      </button>
    </div>
  );
}
