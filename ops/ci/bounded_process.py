"""Supervise one memcap scope; cancellation cleans only its attested unit/session.

With --container (Docker runners, ops/ci/runners/docker) the container's own cgroup is the
bound: admission attests the reporter in the container's root cgroup, and cleanup signals
every process the step started after this supervisor, sparing its own ancestors (the runner)."""
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import tempfile
import time


def start_time(pid):
    stat = Path(f'/proc/{pid}/stat').read_text()
    return int(stat[stat.rfind(')') + 2:].split()[19])


def ancestors(pid):
    chain = set()
    while pid > 0 and pid not in chain:
        chain.add(pid)
        stat = Path(f'/proc/{pid}/stat').read_text()
        pid = int(stat[stat.rfind(')') + 2:].split()[1])
    return chain


def container_signal(signum, since, spared):
    # One job runs per runner container, so a process this step started is this step's.
    for entry in Path('/proc').iterdir():
        if not entry.name.isdigit() or int(entry.name) in spared:
            continue
        try:
            if entry.stat().st_uid == os.getuid() and start_time(int(entry.name)) >= since:
                os.kill(int(entry.name), signum)
        except (OSError, ValueError, IndexError):
            pass


def main(argv):
    container = argv[:1] == ['--container']
    if container:
        if argv[1:2] != ['--'] or not Path('/.dockerenv').is_file():
            print('container supervision outside Docker', file=sys.stderr)
            return 2
        argv = argv[2:]
        since, spared = start_time(os.getpid()), ancestors(os.getpid())
    cancelled = 0

    def cancel(signum, _frame):
        nonlocal cancelled
        cancelled = cancelled or signum

    for signum in (signal.SIGINT, signal.SIGTERM):
        signal.signal(signum, cancel)

    with tempfile.TemporaryDirectory(prefix='jarambit-ci-scope-') as temporary:
        report = Path(temporary) / 'scope.json'
        ack = Path(temporary) / 'admitted'
        env = {**os.environ, 'JARAMBIT_CI_SCOPE_REPORT': str(report),
               'JARAMBIT_CI_SCOPE_ACK': str(ack)}
        process = subprocess.Popen(argv, env=env, start_new_session=True)
        unit = None
        admitted = False
        status = 1

        def unit_signal(name):
            if unit is not None:
                try:
                    subprocess.run(['systemctl', '--user', 'kill', '--kill-whom=all',
                                    '--signal=' + name, unit], capture_output=True, timeout=3)
                except (OSError, subprocess.TimeoutExpired) as error:
                    print('owned scope cleanup failed: ' + str(error), file=sys.stderr)

        def owned_signal(signum):
            if container:
                container_signal(signum, since, spared)

        def group_signal(signum):
            # Popen created this new session before exec. Before admission its
            # bootstrap children cannot execute the body or detach themselves.
            try:
                os.killpg(process.pid, signum)
            except ProcessLookupError:
                pass

        try:
            deadline = time.monotonic() + 30
            while not cancelled and process.poll() is None and time.monotonic() < deadline:
                if report.exists():
                    info = json.loads(report.read_text())
                    pid = info['pid']
                    if type(pid) is not int or os.getsid(pid) != process.pid:
                        raise RuntimeError('scope reporter outside owned session')
                    actual = Path(f'/proc/{pid}/cgroup').read_text().strip().split('::', 1)[1]
                    if container:
                        if actual != info['cgroup'] or actual != '/':
                            raise RuntimeError('unattested container cgroup')
                        admitted = True
                        ack.write_text('admitted\n')
                        break
                    candidate = Path(actual).name
                    if actual != info['cgroup'] or not re.fullmatch(r'run-[A-Za-z0-9_-]+\.scope', candidate):
                        raise RuntimeError('unattested memcap scope')
                    observed = subprocess.check_output(
                        ['systemctl', '--user', 'show', candidate, '-p', 'ControlGroup', '--value'],
                        text=True, timeout=3).strip()
                    if observed != actual:
                        raise RuntimeError('scope user-manager identity mismatch')
                    unit = candidate  # immutable after admission, before body runs
                    admitted = True
                    ack.write_text('admitted\n')
                    break
                time.sleep(0.02)
            if not admitted and not cancelled:
                status = process.poll()
                if status is None:
                    raise RuntimeError('bounded scope admission timed out')
                status = status or 1  # zero exit without admission is not a pass
            else:
                while not cancelled and process.poll() is None:
                    time.sleep(0.02)
                status = process.poll()
            if cancelled:
                status = 128 + cancelled
            elif status is not None and status < 0:
                status = 128 - status
        finally:
            # Unit cleanup includes detached grandchildren. The startup process
            # group closes the pre-admission cancellation window.
            try:
                unit_signal('SIGTERM')
                owned_signal(signal.SIGTERM)
            finally:
                group_signal(signal.SIGTERM)
            try:
                process.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            try:
                unit_signal('SIGKILL')
                owned_signal(signal.SIGKILL)
            finally:
                group_signal(signal.SIGKILL)
                process.wait(timeout=3)
        return status if status is not None else 1


if __name__ == '__main__':
    raise SystemExit(main(sys.argv[1:]))
