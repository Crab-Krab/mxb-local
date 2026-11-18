import { describe, expect, test } from "bun:test";
import { RatingRequests } from "../src/lib/ratingRequests";

describe("rating requests", () => {
  test("a mod already being asked about is not asked again", () => {
    const r = new RatingRequests();
    expect(r.take([1, 2, 3])).toEqual([1, 2, 3]);
    // The grid changed (Load more) before the answer landed.
    expect(r.take([1, 2, 3, 4, 5])).toEqual([4, 5]);
  });

  test("a mod the site did not answer for is asked again by the next grid", () => {
    const r = new RatingRequests();
    const sent = r.take([10873, 2, 3]);
    r.settle(sent, [2, 3]);
    expect(r.take([10873, 2, 3])).toEqual([10873]);
  });

  test("an answered mod stays answered, including one with no votes", () => {
    const r = new RatingRequests();
    const sent = r.take([1, 2]);
    r.settle(sent, [1, 2]);
    expect(r.take([1, 2])).toEqual([]);
  });

  test("a failed request releases every mod it carried", () => {
    const r = new RatingRequests();
    const sent = r.take([1, 2]);
    r.settle(sent, []);
    expect(r.take([2, 1])).toEqual([2, 1]);
  });

  test("clear forgets everything", () => {
    const r = new RatingRequests();
    r.settle(r.take([1]), [1]);
    r.clear();
    expect(r.take([1])).toEqual([1]);
  });
});
