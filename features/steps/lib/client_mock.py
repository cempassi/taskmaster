from __future__ import annotations
import builtins
from enum import Enum
from socket import socket
from features.steps.lib.taskmaster_utils import connect_to_socket, scan_info, scan_status, scan_tasks
from json import dumps
import logging
from typing import Any, Dict, TextIO, Tuple, Union


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

    @staticmethod
    def send_raw_command(command: str) -> Tuple[socket, TextIO]:
        sock = connect_to_socket()
        sockfile = sock.makefile()

        ClientMock.log.debug(f'sending command: {command}')
        sock.send(command.encode())
        return (sock, sockfile)

    @staticmethod
    def send_command(type: ClientCommand, extra: Dict[str, Any] = {}) -> Tuple[socket, TextIO]:
        raw_command = ClientMock.build_command(type, extra)
        command = dumps(raw_command)
        return ClientMock.send_raw_command(command)

    def read_data(self, sock: socket, max_size: int) -> bytes:
        return sock.recv(max_size)

    def readlines(self, sock_file: TextIO, hint: int = -1) -> list[str]:
        return sock_file.readlines(hint)

    def readline(self, sock: socket, size: int) -> str:
        return self.read_data(sock, size).decode()

    def send_list(self) -> Dict[str, object]:
        """send list command to server"""
        sock, _ = ClientMock.send_command(ClientCommand.LIST)
        return scan_tasks(self.readline(sock, 4096))

    def send_start(self, taskname: str):
        """send start command to server"""
        ClientMock.send_command(
            ClientCommand.START, {'id': taskname})

    def send_status(self, taskname: str) -> str:
        """send status command to server"""
        sock, _ = ClientMock.send_command(
            ClientCommand.STATUS, {'id': taskname})
        return scan_status(self.readline(sock, 256))

    def send_stop(self, taskname: str):
        """send stop command to server"""
        ClientMock.send_command(
            ClientCommand.STOP, {'id': taskname})

    def send_info(self, taskname: str):
        """send info command to server"""
        sock, _ = ClientMock.send_command(
            ClientCommand.INFO, {'id': taskname})
        return scan_info(self.readline(sock, 4096))
