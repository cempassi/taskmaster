from __future__ import annotations
from subprocess import Popen
import logging


class ServerProc:

    log = logging.getLogger(__qualname__)  # type: ignore

    def __init__(self, popen: Popen) -> None:
        self.popen = popen
        out, err = popen.communicate()
        self.stream_out = out
        self.stream_err = err

    def is_running(self) -> int | None:
        return self.popen.poll()
