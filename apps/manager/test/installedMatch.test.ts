import { describe, expect, test } from "bun:test";
import { buildInstalledIndex } from "../src/lib/installedMatch";

const index = (...names: string[]) => buildInstalledIndex(names);

describe("installed match", () => {
  test("a numbered sequel is not its predecessor", () => {
    // Only Walnut 2 is on disk: the post for the first Walnut must not show as installed,
    // or its Uninstall button would remove Walnut 2.
    expect(index("Walnut_2.pkz").match("Walnut")).toBeNull();
    expect(index("Walnut.pkz").match("Walnut 2")).toBeNull();
    expect(index("Walnut_1.pkz").match("Walnut 2")).toBeNull();
    expect(index("Walnut_2.pkz").match("Walnut 1")).toBeNull();
    // The same through the word comparison, where every other word agrees.
    expect(index("Mammoth_Mountain_Valley_2.pkz").match("Mammoth Mountain Valley 1")).toBeNull();
    expect(index("Mammoth_Mountain_Valley_2.pkz").match("Mammoth Mountain Valley 2")).toBe(
      "Mammoth_Mountain_Valley_2.pkz",
    );
  });

  test("the same numbered track still matches", () => {
    expect(index("Walnut_2.pkz").match("Walnut 2")).toBe("Walnut_2.pkz");
    expect(index("walnut2.pkz").match("WALNUT 2")).toBe("walnut2.pkz");
  });

  test("a name inside the other matches when no number is cut or added", () => {
    // Author prefix on the file name.
    expect(index("Feulatracks_Farm_MX.pkz").match("Farm Mx")).toBe("Feulatracks_Farm_MX.pkz");
    // A one-word file name long enough to be a name.
    expect(index("HIDEAWAY_MX.pkz").match("HIDEAWAY MX – SC Designs")).toBe("HIDEAWAY_MX.pkz");
  });

  test("a common word in a file name does not claim every post that uses it", () => {
    expect(index("Replica.pnt").match("Factory Honda Replica")).toBeNull();
  });

  test("an abbreviated file name matches the longer post title", () => {
    expect(index("KTM_85_2027_HGS.pnt").match("ktm 85 2027 with hgs front and back pipe")).toBe(
      "KTM_85_2027_HGS.pnt",
    );
  });

  test("model years still have to agree", () => {
    expect(index("Yamaha_YZ450F_2026.pnt").match("Yamaha YZ450F 2027 Factory")).toBeNull();
  });
});
