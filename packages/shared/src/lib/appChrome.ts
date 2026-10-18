/**
 * Make the window behave like an app, not a web page: no browser right-click menu (Back,
 * Reload, Print, Inspect) and none of the browser's own keys (F5, Ctrl+R, Ctrl+P, Alt+Left, the
 * mouse's back and forward buttons, Ctrl+Shift+I). Text fields keep their right-click menu, so
 * copy and paste still work there, and the editing keys (Ctrl+C, Ctrl+V, Ctrl+Z…) are untouched.
 *
 * Call once at start, release builds only, so a dev build can still be reloaded and inspected.
 */
export function installAppChrome(): void {
  const editable = (t: EventTarget | null) =>
    t instanceof HTMLElement && (t.isContentEditable || t.closest("input, textarea, [contenteditable='true']") !== null);

  window.addEventListener(
    "contextmenu",
    (e) => {
      if (!editable(e.target)) e.preventDefault();
    },
    { capture: true },
  );

  window.addEventListener(
    "keydown",
    (e) => {
      const k = e.key.toLowerCase();
      const ctrl = e.ctrlKey || e.metaKey;
      const browserKey =
        k === "f5" ||
        k === "f7" ||
        k === "f12" ||
        k === "browserback" ||
        k === "browserforward" ||
        k === "browserrefresh" ||
        (ctrl && (k === "r" || k === "p" || k === "u" || k === "f" || k === "g" || k === "j" || k === "h" || k === "s")) ||
        (ctrl && e.shiftKey && (k === "i" || k === "j" || k === "c" || k === "delete")) ||
        (e.altKey && (k === "arrowleft" || k === "arrowright" || k === "home"));
      if (browserKey) e.preventDefault();
    },
    { capture: true },
  );

  // The mouse's back and forward buttons (3 and 4) navigate the page history.
  const noNav = (e: MouseEvent) => {
    if (e.button === 3 || e.button === 4) e.preventDefault();
  };
  window.addEventListener("mouseup", noNav, { capture: true });
  window.addEventListener("auxclick", noNav, { capture: true });
}
