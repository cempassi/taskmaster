from features.steps.lib.utils import Namespace, NamespaceLock
from socket import socket
from typing import List
from features.steps.lib.taskmaster_utils import connect_to_socket, listen_to_socket
import logging
from threading import Thread, Lock
from select import select


log = logging.getLogger('server_mock')


class ServerMock:

    log = log.getChild(__qualname__)

    def __init__(self) -> None:
        self.sock = listen_to_socket()
        self.sockfile = self.sock.makefile()
        self.sock.listen(2)
        self.lock = Lock()
        self.shared = NamespaceLock(
            self.lock, running=True, log=self.log, sock=self.sock)
        self.thread = Thread(
            None, worker, name=f'Thread-{ServerMock.__name__}', args=[self.shared])
        self.thread.start()
        self.log.debug('server mock created')

    def __del__(self) -> None:
        self.log.debug('destructing')
        self.shared.running = False
        self.thread.join(0.5)


def worker(shared: Namespace):
    shared.log.error('thread started')
    clients: List[socket] = []
    while shared.running:
        shared.log.debug(f'continue {shared.running}')
        r, _w, _x = select([*clients, shared.sock], [], [], 0.5)
        shared.log.debug(f'select event: {r}')
        for read_ready in r:
            if read_ready == shared.sock:
                csock, _caddr = shared.sock.accept()
                shared.log.info(f'new client: {csock}')
                clients.append(csock)
            elif read_ready in clients:
                csock: socket = read_ready
                raw_msg = csock.recv(4096)
                shared.log.debug(f'client={csock} raw_msg={raw_msg}')
                if len(raw_msg) == 0:
                    shared.log.info(f'client quit: {csock}')
                    clients.remove(csock)
                    csock.close()

    shared.log.error('thread stopping')
