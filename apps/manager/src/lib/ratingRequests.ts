/**
 * Which mods still need their score asked for, across every listing the grid shows.
 *
 * A mod is held as asked from the moment its request is sent, so a grid that changes while
 * the request is out (a cached page refreshed, Load more, a sort switch) does not ask twice.
 * Once the answer lands, any mod the site did not answer for is released again, so the next
 * grid that shows it asks once more. Before, a failed or discarded answer left the mod held
 * for good and its card showed no stars until the player pressed reload.
 */
export class RatingRequests {
  private asked = new Set<number>();

  /** The ids from `ids` not yet asked for, now held as asked. */
  take(ids: number[]): number[] {
    const wanted: number[] = [];
    for (const id of ids) {
      if (this.asked.has(id)) continue;
      this.asked.add(id);
      wanted.push(id);
    }
    return wanted;
  }

  /**
   * Records the answer to a `take`. Ids missing from `answered` (the site refused or the
   * request failed) are released, so a later grid asks again.
   */
  settle(sent: number[], answered: Iterable<number>): void {
    const got = new Set(answered);
    for (const id of sent) if (!got.has(id)) this.asked.delete(id);
  }

  /** Forget everything, for an explicit reload. */
  clear(): void {
    this.asked.clear();
  }
}
