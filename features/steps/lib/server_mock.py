from features.steps.lib.taskmaster_utils import connect_to_socket, listen_to_socket
import logging


class ServerMock:

    log = logging.getLogger(__qualname__)

    def __init__(self) -> None:
        self.sock = listen_to_socket()
        self.sockfile = self.sock.makefile()

    def __del__(self) -> None:
        self.log.debug("destructing")
        pass
