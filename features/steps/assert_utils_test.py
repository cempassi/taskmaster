from sys import stderr
from typing import Any, Dict
import unittest

from asserts import assert_equal
from features.steps.assert_utils import DEFAULT_CONFIG_TASK, assert_task

DEFAULT_TASK = {
    **DEFAULT_CONFIG_TASK,
    'stopsignal': 'SIGTERM',
}


def mock_task(partial_task: Dict[str, Any]):
    return {'cmd': 'echo foo', **DEFAULT_TASK, **partial_task}


def mock_config_task(partial_config: Dict[str, Any]):
    return {'cmd': 'echo foo', **DEFAULT_CONFIG_TASK, **partial_config}


class TestAssertTask(unittest.TestCase):

    def test_same_object_equal(self):
        data = {
            'cmd': 'echo foo'
        }
        task = mock_task(data)
        config = mock_config_task(data)
        assert_task(task, config)

    def _test_config(self, equal_base: Dict[str, Any], differ_base: Dict[str, Any]):
        task = mock_task(equal_base)
        equal = mock_config_task(equal_base)
        diff = mock_config_task(differ_base)
        assert_task(task, equal)
        with self.assertRaises(AssertionError):
            assert_task(task, diff)

    def test_cmd(self):
        self._test_config({'cmd': 'echo foo'}, {'cmd': 'echo bar'})

    def test_umask(self):
        self._test_config({'umask': 0o45}, {'umask': 0o77})

    def test_numprocess(self):
        self._test_config({'numprocess': 42}, {'numprocess': 51})

    def test_autostart(self):
        self._test_config({'autostart': True}, {'autostart': False})

    def test_workingdir(self):
        self._test_config({'workingdir': '/yolo'}, {'workingdir': '/foo'})

    def test_retry(self):
        self._test_config({'retry': 5}, {'retry': 6})

    def test_restart(self):
        self._test_config({'restart': 'on-error'}, {'restart': 'always'})

    def test_exitcodes(self):
        self._test_config({'exitcodes': [0, 1, 2]}, {
                          'exitcodes': [0, 1, 2, 24]})

    def test_successdelay(self):
        self._test_config({'successdelay': 20}, {'successdelay': 5})

    def test_stopsignal(self):
        self._test_config({'stopsignal': 'QUIT'}, {'stopsignal': 'KILL'})

    def test_stopdelay(self):
        self._test_config({'stopdelay': 5}, {'stopdelay': 4})

    def test_stdout(self):
        self._test_config({'stdout': 'file.log'}, {'stdout': 'not.log'})

    def test_stderr(self):
        self._test_config({'stderr': 'yolo.log'}, {'stderr': 'yes.log'})

    def test_env(self):
        self._test_config({'env': {'foo': 'bar', 'path': 'test'}}, {
                          'env': {'foo': 'bar', 'path': 'not'}})

    def test_uid(self):
        self._test_config({'uid': 1089}, {'uid': 1042})

    def test_gid(self):
        self._test_config({'gid': 1078}, {'gid': 1029})
