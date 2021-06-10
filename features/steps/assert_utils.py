from typing import Any, Dict
from asserts import assert_equal, assert_true
import logging

log = logging.getLogger('assert_utils')


def assert_task(got, wanted):
    l = log.getChild(assert_task.__name__)
    l.debug(f'compare {got} to {wanted}')
    for key in ['cmd', 'umask', 'autostart', 'numprocess', 'workingdir', 'stopsignal', 'stopdelay', 'stdout', 'stderr', 'retry', 'successdelay', 'exitcodes', 'restart', 'env', 'uid', 'gid']:
        vgot = got[key]
        vwanted = wanted[key]
        l.debug(f'for key {key}: got <{vgot}> wanted <{vwanted}>')
        if key == 'stopsignal':
            assert_true(vgot.startswith("SIG"))
            assert_equal(vgot[3:], vwanted)
        else:
            assert_equal(vgot, vwanted)
    assert_true(False)


def assert_tasks(got: Dict[str, Any], wanted: Dict[str, Any]):
    keys = set()
    keys.update(got.keys())
    keys.update(wanted.keys())
    for key in keys:
        vgot = got.get(key)
        vwanted = wanted.get(key)

        assert_true(vgot, msg_fmt=f'missing value for {key}')
        assert_true(vwanted, msg_fmt=f'extra value from got with key {key}')
        assert_task(got[key], wanted[key])
