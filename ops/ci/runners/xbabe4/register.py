"""Read one short-lived registration token from stdin, without retaining it."""
from pathlib import Path
import subprocess
import sys


if Path('.runner').exists():
    raise SystemExit('Runner already configured; preserve its existing state volume.')
token = sys.stdin.read().strip()
if not token or any(character.isspace() for character in token):
    raise SystemExit('Expected one repository registration token on stdin.')
raise SystemExit(subprocess.call([
    './config.sh', '--unattended', '--url', 'https://github.com/neverhuman/jarambit',
    '--token', token, '--name', 'jarambit-xbabe4',
    '--labels', 'xbabe,general,xbabe4,docker', '--work', '_work',
]))
