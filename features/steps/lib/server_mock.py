import logging


class ServerMock:

    log = logging.getLogger(__qualname__)

    def __init__(self) -> None:
        pass

    def __del__(self) -> None:
        self.log.debug("destructing")
        pass
