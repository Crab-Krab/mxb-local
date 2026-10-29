"""Refresh only the selected TLS packages, then verify the resulting lockfile."""
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parent.parent
FLOORS = {
    'rustls': (0, 23, 46),
    'rustls-webpki': (0, 103, 16),
    'webpki-roots': (1, 0, 10),
    'webpki-root-certs': (1, 0, 10),
    'h2': (0, 4, 16),
}

def packages():
    return tomllib.loads((ROOT/'Cargo.lock').read_text(encoding='utf-8'))['package']

def meets_floors(resolved):
    for name, floor in FLOORS.items():
        matches = [p for p in resolved if p['name'] == name and
                   not (name == 'webpki-roots' and p['version'].startswith('0.'))]
        if not matches:
            return False
        for package in matches:
            version = package['version']
            if '-' in version or tuple(int(n) for n in version.split('.')[:3]) < floor:
                return False
    return True

if __name__ == '__main__':
    if meets_floors(packages()):
        print('Using the checked-in, verified TLS lockfile; no dependency updates requested.')
        raise SystemExit(0)
    selected = []
    for package in packages():
        name = package['name']
        if name not in FLOORS:
            continue
        # Keep the webpki-roots 0.26 compatibility wrapper on its own version line.
        if name == 'webpki-roots' and package['version'].startswith('0.'):
            continue
        selected.extend(['-p', f"{name}@{package['version']}"])
    subprocess.run(['cargo', 'update', *selected], cwd=ROOT, check=True)
    if not meets_floors(packages()):
        raise SystemExit('Refusing a lockfile that does not satisfy every TLS security floor.')
    print('TLS security floors satisfied. Cargo.lock is ready for review and commit.')
