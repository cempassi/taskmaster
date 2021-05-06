from __future__ import annotations
from features.steps.lib.taskmaster_utils import TASKMASTER_PATH, get_taskmaster_args
from subprocess import PIPE, Popen
import logging
from types import SimpleNamespace
from typing import List


log = logging.getLogger('server_proc')


def get_server_args(config: SimpleNamespace) -> List[str]:
    l = log.getChild(get_server_args.__name__)
    args = get_taskmaster_args(config)
    args.append('server')
    if isinstance(config.configfile, str):
        args.extend(['--config', config.configfile])
    l.debug(f'args={args}')
    return args


class ServerProc:

    log = log.getChild(__qualname__)  # type: ignore

    def __init__(self, config: str, verbose: str) -> None:
        cfg = SimpleNamespace(configfile=config, verbose=verbose)
        self.args = get_server_args(cfg)
        self.proc = Popen(self.args, executable=TASKMASTER_PATH,
                          stdout=PIPE, stderr=PIPE)

    def close(self):
        pass

    def is_running(self) -> int | None:
        return self.proc.poll()
