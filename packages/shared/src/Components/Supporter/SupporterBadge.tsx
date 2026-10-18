import { Coffee } from "lucide-react";
import { useT } from "../../i18n/context";
import { cn } from "../../lib/utils";

/**
 * The Supporter badge: a small coffee cup, with the word beside it unless `compact`. Shown next
 * to an account, a live share or a server whose owner is a supporter (a Buy Me a Coffee
 * membership). The Style Pack doesn't carry it.
 */
export default function SupporterBadge({
  compact = false,
  className,
}: {
  compact?: boolean;
  className?: string;
}) {
  const t = useT();
  const label = t("supporter.badge");
  return (
    <span
      title={label}
      aria-label={label}
      className={cn(
        "inline-flex shrink-0 items-center gap-1 font-cond text-[10.5px] font-semibold uppercase tracking-[0.08em] text-warning",
        className,
      )}
    >
      <Coffee className="size-3" />
      {!compact && label}
    </span>
  );
}
