from __future__ import annotations
from os import stat
from features.steps.lib.utils import Namespace
from features.steps.lib.taskmaster_utils import TASKMASTER_PATH, TASKMASTER_SOCK, get_taskmaster_args
from subprocess import Popen, STDOUT
from inotify.adapters import Inotify
import logging


log = logging.getLogger('server_proc')


def get_server_args(config: Namespace) -> list[str]:
    l = log.getChild(get_server_args.__name__)
    args = get_taskmaster_args(config)
    args.append('server')
    if isinstance(config.configfile, str):
        args.append(config.configfile)
    if isinstance(config.format, str):
        args.extend(['-f', config.format])
    l.debug(f'args={args}')
    return args


class ServerProc:

    log = log.getChild(__qualname__)  # type: ignore

    @staticmethod
    def prepare_to_wait() -> Inotify:
        from os.path import dirname

        i = Inotify()
        i.add_watch(dirname(TASKMASTER_SOCK))
        return i

    @staticmethod
    def wait_for_server_to_be_ready(watcher: Inotify):
        from os.path import join
        for event in watcher.event_gen(yield_nones=False, timeout_s=2):
            (_, type_names, path, filename) = event
            filepath = join(path, filename)
            ServerProc.log.debug(
                f'type_names={type_names}, path={path}, filename={filename}')
            if filepath == TASKMASTER_SOCK and 'IN_CREATE' in type_names:
                return
        raise TimeoutError(
            'server take too much time to start')

    def __init__(self, config: str, verbose: str, format: str) -> None:
        cfg = Namespace(configfile=config, verbose=verbose, format=format)
        self.log.debug(f'server config: {cfg}')
        self.args = get_server_args(cfg)
        watch = ServerProc.prepare_to_wait()
        self.proc = Popen(self.args, executable=TASKMASTER_PATH,
                          stdout=open('server.log', 'w'), stderr=STDOUT)
        self.log.debug(f'waiting for server')
        ServerProc.wait_for_server_to_be_ready(watch)

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
