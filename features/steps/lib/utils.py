from typing import Any, Union


class Namespace(dict):
    def __getattr__(self, attr) -> Union[Any, None]:
        return self.get(attr)
