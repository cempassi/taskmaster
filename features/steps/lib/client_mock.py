from __future__ import annotations
from enum import Enum
from socket import socket
from features.steps.lib.taskmaster_utils import connect_to_socket, scan_tasks
from json import dumps
import logging
from typing import Any, Dict, TextIO, Union


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
        pass

    @staticmethod
    def build_command(name: str, extra: Dict[str, Any] = {}) -> Dict[str, Any]:
        cmd = {**extra, 'type': name}
        ClientMock.log.debug(f'cmd={cmd}')
        return cmd

    def send_command(self, command: str) -> Union[socket, TextIO]:
        sock = connect_to_socket()
        sockfile = sock.makefile()

        self.log.debug(f'sending command: {command}')
        sock.send(command.encode())
        return (sock, sockfile)

    def read_data(self, sock: socket, max_size: int) -> bytes:
        return sock.recv(max_size)

    def readlines(self, sock_file: TextIO, hint: int = -1) -> list[str]:
        return sock_file.readlines(hint)

    def readline(self, sock: socket, size: int) -> str:
        return self.read_data(sock, size).decode()

    def send_list(self) -> Dict[str, object]:
        """send list command to server"""
        raw_command = ClientMock.build_command(ClientCommand.LIST)
        command = dumps(raw_command)
        sock, _ = self.send_command(command)
        return scan_tasks(self.readline(sock, 4096))

    def send_start(self, taskname: str):
        """send start command to server"""
        raw_command = ClientMock.build_command(
            ClientCommand.START, {'id': taskname})
        command = dumps(raw_command)
        self.send_command(command)
