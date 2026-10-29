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

if __name__ == '__main__':
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
    for package in packages():
        name = package['name']
        if name not in FLOORS:
            continue
        if name == 'webpki-roots' and package['version'].startswith('0.'):
            continue
        version = tuple(int(n) for n in package['version'].split('.')[:3])
        if version < FLOORS[name]:
            raise SystemExit(f"Refusing vulnerable TLS package: {name} {package['version']}")
    print('TLS security floors satisfied. Cargo.lock is ready for review and commit.')
