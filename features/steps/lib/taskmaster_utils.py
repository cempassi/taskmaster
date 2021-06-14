from __future__ import annotations
from types import SimpleNamespace
from typing import Any, Dict, List
import logging

log = logging.getLogger('taskmaster_utils')

TASKMASTER_PATH = 'target/release/taskmaster'
TASKMASTER_SOCK = '/tmp/taskmaster.sock'


def get_taskmaster_args(config: SimpleNamespace) -> List[str]:
    l = log.getChild(get_taskmaster_args.__name__)
    args = ['taskmaster']
    # if isinstance(config.verbose, str):
    #     args.extend(['--verbose', config.verbose])
    if isinstance(config.logfile, str):
        args.extend(['--log-file', config.logfile])
    l.debug(f'args={args}')
    return args


def connect_to_socket():
    import socket
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(TASKMASTER_SOCK)
    return sock


def listen_to_socket():
    import socket
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.bind(TASKMASTER_SOCK)
    return sock


def scan_yaml_reply(stream: str) -> Any:
    from yaml import Loader, load
    l = log.getChild(scan_tasks.__name__)
    data = load(stream, Loader=Loader)
    return data


def scan_tasks(stream: str) -> Dict[str, Any]:
    return scan_yaml_reply(stream)


def scan_status(stream: str) -> str:
    data = scan_yaml_reply(stream)
    return data['status']


def scan_info(stream: str) -> Dict[str, Any]:
    data = scan_yaml_reply(stream)
    return data['task']
