import { describe, expect, test } from "bun:test";
import { HOME_VIEW, RAIL, supportedView } from "../src/Components/Shell/nav";
import { ACCOUNT_SUPPORTED, ONLINE_SUPPORTED, SHARE_SUPPORTED } from "../src/lib/localFork";

const views = () =>
  RAIL.flatMap((item) => [item.view, ...(item.tabs ?? []).map((t) => t.view), ...(item.owns ?? [])]);

describe("features MXB Local leaves out", () => {
  test("are switched off", () => {
    expect(ONLINE_SUPPORTED).toBe(false);
    expect(ACCOUNT_SUPPORTED).toBe(false);
    expect(SHARE_SUPPORTED).toBe(false);
  });

  test("the menu has no Online, Ranked or My mods", () => {
    expect(RAIL.map((i) => i.id)).not.toContain("servers");
    expect(views()).not.toContain("servers");
    expect(views()).not.toContain("ranked");
    expect(views()).not.toContain("mymods");
    // Garage stays, with all three tabs.
    const garage = RAIL.find((i) => i.id === "garage");
    expect(garage?.tabs?.map((t) => t.view)).toEqual(["locker", "presets", "manage"]);
  });

  test("the app opens on Mods", () => {
    expect(HOME_VIEW).toBe("browse");
  });

  test("an old link to a left-out screen lands on the one that replaced it", () => {
    expect(supportedView("servers")).toBe("browse");
    expect(supportedView("ranked")).toBe("browse");
    expect(supportedView("mymods")).toBe("library");
    expect(supportedView("locker")).toBe("locker");
    expect(supportedView("settings")).toBe("settings");
  });
});
