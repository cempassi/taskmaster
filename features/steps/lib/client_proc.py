from __future__ import annotations
from os import SEEK_END, SEEK_SET
from features.steps.lib.utils import Namespace
from features.steps.lib.taskmaster_utils import TASKMASTER_PATH, get_taskmaster_args
import logging
from subprocess import PIPE, Popen

log = logging.getLogger('client_proc')


def get_client_args(config: Namespace) -> list[str]:
    l = log.getChild(get_client_args.__name__)
    args = get_taskmaster_args(config)
    args.append('client')
    l.debug(f'args={args}')
    return args


class ClientProc:
    log = log.getChild(__qualname__)  # type: ignore

    def __init__(self, **kwargs) -> ClientProc:
        cfg = Namespace(**kwargs)
        self.args = get_client_args(cfg)
        self.proc = Popen(self.args, executable=TASKMASTER_PATH, stdin=PIPE,
                          stdout=PIPE, stderr=PIPE)

    def __str__(self) -> str:
        return ' '.join(self.args)

    def close(self):
        self.proc.terminate()
        try:
            self.proc.wait(timeout=2)
        except TimeoutError:
            self.proc.kill()

    def is_running(self) -> bool:
        return self.proc.poll() is None

    def write(self, data: str) -> int:
        return self.proc.stdin.write(data.encode())

    def read_stdout(self) -> bytes:
        return self.proc.stdout.read()

    def readline_stdout(self, limit: int = -1) -> bytes:
        return self.proc.stdout.readline(limit=limit)

    def readlines_stdout(self, hint: int = -1) -> list[bytes]:
        return self.proc.stdout.readlines(hint)

    def readlines_stderr(self, hint: int = -1) -> list[bytes]:
        return self.proc.stderr.readlines(hint)

    def seek_stdout(self, offset, whence=SEEK_SET) -> int:
        return self.proc.stdout.seek(offset, whence)

    def flush_stdout(self):
        if self.proc.stdout.seekable():
            self.seek_stdout(0, SEEK_END)
        else:
            lines = self.readlines_stdout()
            self.log.debug(f'skipped lines: {lines}')
