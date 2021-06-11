from __future__ import annotations
from enum import Enum
from features.steps.lib.taskmaster_utils import connect_to_socket, scan_tasks
from json import dumps
import logging
from typing import Any, Dict


log = logging.getLogger('client_mock')


class ClientCommand(str, Enum):
    LIST = 'List'
    START = 'Start'
    RELOAD = 'Reload'
    STOP_SERVER = 'Quit'
    INFO = 'Info'
    STOP = 'Stop'
    STATUS = 'Status'
    RESTART = 'Restart'

    def __str__(self) -> str:
        return str(self.value)


class ClientMock:
    log = log.getChild(__qualname__)  # type: ignore

    def __init__(self) -> None:
        self.sock = connect_to_socket()
        self.sock_file = self.sock.makefile()

    @staticmethod
    def build_command(name: str, extra: Dict[str, Any] = {}) -> Dict[str, Any]:
        cmd = {**extra, 'type': name}
        ClientMock.log.debug(f'cmd={cmd}')
        return cmd

    def send_command(self, command: str):
        self.log.debug(f'sending command: {command}')
        self.sock.send(command.encode())

    def read_data(self, max_size: int) -> bytes:
        return self.sock.recv(max_size)

    def readlines(self, hint: int = -1) -> list[str]:
        return self.sock_file.readlines(hint)

    def readline(self, size: int) -> str:
        return self.read_data(size).decode()

    def send_list(self) -> Dict[str, object]:
        """send list command to server"""
        command = dumps(ClientMock.build_command(ClientCommand.LIST))
        self.send_command(command)
        return scan_tasks(self.readline(4096))
