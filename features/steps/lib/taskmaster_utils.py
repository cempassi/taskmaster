
from types import SimpleNamespace
from typing import List
import logging

log = logging.getLogger('taskmaster_utils')

TASKMASTER_PATH = 'target/release/taskmaster'


def get_taskmaster_args(config: SimpleNamespace) -> List[str]:
    l = log.getChild(get_taskmaster_args.__name__)
    args = ['taskmaster']
    if isinstance(config.verbose, str):
        args.extend(['--verbose', config.verbose])
    l.debug(f'args={args}')
    return args
