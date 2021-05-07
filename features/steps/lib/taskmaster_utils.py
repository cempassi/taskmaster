
from types import SimpleNamespace
from typing import List
import logging

log = logging.getLogger('taskmaster_utils')

TASKMASTER_PATH = 'target/release/taskmaster'
TASKMASTER_SOCK = '/tmp/taskmaster.sock'


def get_taskmaster_args(config: SimpleNamespace) -> List[str]:
    l = log.getChild(get_taskmaster_args.__name__)
    args = ['taskmaster']
    if isinstance(config.verbose, str):
        args.extend(['--verbose', config.verbose])
    l.debug(f'args={args}')
    return args


def connect_to_socket():
    import socket
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(TASKMASTER_SOCK)
    return sock
