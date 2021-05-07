from features.steps.lib.client_mock import ClientMock
from features.steps.lib.taskmaster_utils import connect_to_socket
from os import write
from typing import List
from behave import then, register_type, use_step_matcher
from features.steps.lib.pattern import parse_int
import logging

log = logging.getLogger('then')

register_type(Int=parse_int, String=str)
use_step_matcher('cfparse')


@then('server has read {task_to_read:Int} tasks')
def assert_tasks_read(ctx, task_to_read):
    mock = ClientMock()
    l = log.getChild(assert_tasks_read.__name__)
    l.debug(f'task_to_read={task_to_read}')
    mock.send_list()
    data = mock.readlines(4096)
    l.debug(f'data={data}')
    raise NotImplementedError


@then('server is still running')
def assert_server_running(ctx):
    l = log.getChild(assert_server_running.__name__)
    isrunning = ctx.server.is_running()
    l.debug(f'isrunning={isrunning}')
    assert isrunning


@then('client is still running')
def assert_client_running(ctx):
    l = log.getChild(assert_client_running.__name__)
    stderr_lines = ctx.client.readlines_stderr()
    l.debug(f'err_lines={stderr_lines}')
    isrunning = ctx.client.is_running()
    l.debug(f'isrunning={isrunning}')
    assert isrunning


@then('the tasks are named {task_names:String+}')
def assert_task_names(ctx, task_names: List[str]):
    l = log.getChild(assert_task_names.__name__)
    l.debug(f'task_names={task_names}')
    raise NotImplementedError


@then('we read the help command output')
def check_help_command_output(ctx):
    l = log.getChild(check_help_command_output.__name__)
    lines = ctx.client.readlines_stdout()
    l.debug(f'lines={lines}')
    raise NotImplementedError
