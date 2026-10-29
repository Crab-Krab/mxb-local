"""Collect current OSV advisories for the exact Rust and Bun lockfiles.

This produces review evidence, not a claim that all reported packages are shipped
or exploitable. Withdrawn and informational RustSec records stay in the report.
The release publisher must review active findings before publishing a build.
"""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import json
import re
import time
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / '.security-results'


def request(url, payload=None):
    data = json.dumps(payload).encode() if payload is not None else None
    for attempt in range(3):
        try:
            req = urllib.request.Request(url, data=data, headers={
                'Content-Type': 'application/json', 'User-Agent': 'MXB-Local-dependency-review'})
            with urllib.request.urlopen(req, timeout=60) as response:
                return json.load(response)
        except Exception:
            if attempt == 2:
                raise
            time.sleep(attempt + 1)


def main():
    queries = []
    for package in tomllib.loads((ROOT / 'Cargo.lock').read_text(encoding='utf-8'))['package']:
        if package.get('source', '').startswith('registry+'):
            queries.append({'package': {'name': package['name'], 'ecosystem': 'crates.io'},
                            'version': package['version']})
    locked = (ROOT / 'bun.lock').read_text(encoding='utf-8')
    for name, version in re.findall(r'"[^"\n]+":\s*\["((?:@[^/"\n]+/)?[^@"\n]+)@([^"\n]+)"', locked):
        query = {'package': {'name': name, 'ecosystem': 'npm'}, 'version': version}
        if query not in queries:
            queries.append(query)
    assert len(queries) > 500, 'Unexpectedly small lockfile inventory'
    results = []
    for start in range(0, len(queries), 200):
        batch = queries[start:start + 200]
        answer = request('https://api.osv.dev/v1/querybatch', {'queries': batch})['results']
        assert len(answer) == len(batch)
        results.extend(dict(query=q, **hit) for q, hit in zip(batch, answer))
    hits = [row for row in results if row.get('vulns')]
    ids = sorted({v['id'] for hit in hits for v in hit['vulns']})
    with ThreadPoolExecutor(max_workers=6) as pool:
        details = list(pool.map(lambda key: request('https://api.osv.dev/v1/vulns/' + key), ids))
    by_id = {row['id']: row for row in details}
    summary = []
    for hit in hits:
        package = hit['query']
        for match in hit['vulns']:
            advisory = by_id[match['id']]
            informational = advisory.get('database_specific', {}).get('informational')
            status = 'withdrawn' if advisory.get('withdrawn') else 'informational' if informational else 'review-required'
            summary.append({'package': package, 'id': advisory['id'], 'status': status,
                            'summary': advisory.get('summary', ''), 'informational': informational})
    OUTPUT.mkdir(exist_ok=True)
    report = {'queried': len(queries), 'hits': hits, 'summary': summary, 'advisories': details}
    (OUTPUT / 'dependency-advisories.json').write_text(json.dumps(report, indent=2), encoding='utf-8')
    print(f'Checked {len(queries)} locked package versions; {len(ids)} advisory records to classify.')
    for row in summary:
        p = row['package']
        print(f"{row['status']}: {p['package']['ecosystem']} {p['package']['name']}@{p['version']} "
              f"{row['id']} {row['summary']}")
    count = sum(row['status'] == 'review-required' for row in summary)
    print(f'{count} active package/advisory matches require release review. '
          'The inventory includes build-only and other-platform dependencies.')


if __name__ == '__main__':
    main()
