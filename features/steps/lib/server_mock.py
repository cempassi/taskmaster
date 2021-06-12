from socket import socket
from typing import List
from features.steps.lib.taskmaster_utils import connect_to_socket, listen_to_socket
import logging
from threading import Thread
from select import select


log = logging.getLogger('server_mock')


class ServerMock:

    log = log.getChild(__qualname__)

    def __init__(self) -> None:
        self.sock = listen_to_socket()
        self.sockfile = self.sock.makefile()
        self.sock.listen(2)
        self.continue_running = True
        self.thread = Thread(None, self.run, f'Thread-{ServerMock.__name__}')
        self.thread.start()
        self.log.debug('server mock created')

    def __del__(self) -> None:
        self.log.debug('destructing')
        self.continue_running = False
        self.thread.join(5)
        pass

    def run(self):
        self.log.error('thread started')
        clients: List[socket] = []
        while self.continue_running:
            r, _w, _x = select([*clients, self.sock], [], [], 0.2)
            self.log.debug(f'select event: {r}')
            for read_ready in r:
                if read_ready == self.sock:
                    csock, _caddr = self.sock.accept()
                    self.log.info(f'new client: {csock}')
                    clients.append(csock)
                elif read_ready in clients:
                    csock: socket = read_ready
                    raw_msg = csock.recv(4096)
                    self.log.debug(f'client={csock} raw_msg={raw_msg}')
                    if len(raw_msg) is 0:
                        self.log.info(f'client quit: {csock}')
                        clients.remove(csock)
                        csock.close()
