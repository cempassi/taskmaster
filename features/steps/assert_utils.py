from typing import Any, Dict
from asserts import assert_equal, assert_true
import logging

log = logging.getLogger('assert_utils')

DEFAULT_CONFIG_TASK = {
    'umask': 0,
    'numprocess': 1,
    'autostart': False,
    'workingdir': '.',
    'retry': 0,
    'restart': 'never',
    'exitcodes': [0],
    'successdelay': 0,
    'stopsignal': 'TERM',
    'stopdelay': 2,
    'stdout': '/dev/null',
    'stderr': '/dev/null',
    'env': {},
    'uid': None,
    'gid': None,
}


def assert_task(got, wanted):
    l = log.getChild(assert_task.__name__)
    l.debug(f'compare {got} to {wanted}')
    for key in ['cmd', 'umask', 'autostart', 'numprocess', 'workingdir', 'stopsignal', 'stopdelay', 'stdout', 'stderr', 'retry', 'successdelay', 'exitcodes', 'restart', 'env', 'uid', 'gid']:
        vgot = got[key]
        vwanted = wanted.get(key)
        if vwanted is None:
            log.debug(f'no expected value for {key}, fallback to default')
            vwanted = DEFAULT_CONFIG_TASK[key]
        l.debug(f'for key {key}: got <{vgot}> wanted <{vwanted}>')
        if key == 'stopsignal':
            assert_true(vgot.startswith("SIG"),
                        msg_fmt='check signal name begin with SIG')
            assert_equal(vgot[3:], vwanted)
        else:
            assert_equal(vgot, vwanted)


def assert_tasks(got: Dict[str, Any], wanted: Dict[str, Any]):
    keys = set()
    keys.update(got.keys())
    keys.update(wanted.keys())
    for key in keys:
        vgot = got.get(key)
        vwanted = wanted.get(key)

        assert_true(vgot, msg_fmt=f'expected task with key "{key}"')
        assert_true(vwanted, msg_fmt=f'unexpected task with key "{key}"')
        assert_task(got[key], wanted[key])
