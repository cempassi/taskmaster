from features.steps.lib.utils import Namespace
from typing import List
from features.steps.lib.taskmaster_utils import TASKMASTER_PATH, get_taskmaster_args
import logging
from subprocess import PIPE, Popen
from types import SimpleNamespace

log = logging.getLogger('client_proc')


def get_client_args(config: Namespace) -> List[str]:
    l = log.getChild(get_client_args.__name__)
    args = get_taskmaster_args(config)
    args.append('client')
    l.debug(f'args={args}')
    return args


class ClientProc:
    log = log.getChild(__qualname__)  # type: ignore

    def __init__(self) -> None:
        cfg = Namespace()
        self.args = get_client_args(cfg)
        self.proc = Popen(self.args, executable=TASKMASTER_PATH,
                          stdout=PIPE, stderr=PIPE)

    def close(self):
        self.proc.terminate()
        try:
            self.proc.wait(timeout=2)
        except TimeoutError:
            self.proc.kill()

    def is_running(self) -> bool:
        return self.proc.poll() is None
