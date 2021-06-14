from threading import Lock
from typing import Any, Union


class Namespace(dict):
    def __getattr__(self, attr) -> Union[Any, None]:
        return self.get(attr)


class NamespaceLock(Namespace):
    def __init__(self, lock: Lock, *args, **kwargs):
        self.lock = lock
        super().__init__(*args, **kwargs)

    def __getattr__(self, attr) -> Union[Any, None]:
        self.lock.acquire()
        data = super().__getattr__(attr)
        self.lock.release()
        return data
