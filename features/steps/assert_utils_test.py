from typing import Any, Dict
import unittest
from features.steps.assert_utils import DEFAULT_CONFIG_TASK, assert_task

DEFAULT_TASK = {
    **DEFAULT_CONFIG_TASK,
    'stopsignal': 'SIGTERM',
}


def mock_task(partial_task: Dict[str, Any]):
    return {**DEFAULT_TASK, **partial_task}


def mock_config_task(partial_config: Dict[str, Any]):
    return {**DEFAULT_CONFIG_TASK, **partial_config}


class TestAssertTask(unittest.TestCase):
    def test_same_object_equal(self):
        data = {
            'cmd': 'echo foo'
        }
        task = mock_task(data)
        config = mock_config_task(data)
        assert_task(task, config)

    def test_cmd_differ(self):
        task = mock_task({'cmd': 'echo foo'})
        config = mock_task({'cmd': 'echo bar'})
        with self.assertRaises(AssertionError):
            assert_task(task, config)
