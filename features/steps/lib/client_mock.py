from __future__ import annotations
from enum import Enum
from features.steps.lib.taskmaster_utils import connect_to_socket
from json import dumps
import logging


log = logging.getLogger('client_mock')


class ClientCommand(Enum):
    LIST = 'List'

    def __str__(self) -> str:
        return str(self.value)


class ClientMock:
    log = log.getChild(__qualname__)  # type: ignore

    def __init__(self) -> None:
        self.sock = connect_to_socket()
        self.sock_file = self.sock.makefile()

    def send_command(self, command: str):
        self.log.debug(f'sending command: {command}')
        self.sock.send(command.encode())

    def send_list(self):
        """send list command to server"""
        command = dumps(str(ClientCommand.LIST))
        self.send_command(command)

    def read_data(self, max_size: int) -> bytes:
        return self.sock.recv(max_size)

    def readlines(self, hint: int = -1) -> list[str]:
        return self.sock_file.readlines(hint)
