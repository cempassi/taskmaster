from typing import List
from behave import then, register_type, use_step_matcher
from features.steps.lib.pattern import parse_int
import logging
from asserts import assert_equal

log = logging.getLogger('then')

register_type(Int=parse_int, String=str)
use_step_matcher('cfparse')


@then('server has read the good amount of tasks')
def assert_tasks_read(ctx):
    l = log.getChild(assert_tasks_read.__name__)
    l.debug(f'tasks={ctx.read_tasks}')
    # assert len(ctx.read_tasks.values()) == task_to_read
    assert_equal(len(ctx.read_tasks.keys()), len(ctx.config_file_data.keys()))


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


@then('server has read the named tasks')
def assert_task_names(ctx):
    l = log.getChild(assert_task_names.__name__)
    task_names = ctx.config_file_data.keys()
    l.debug(f'task_names={task_names}')
    read_tasks_name = list(ctx.read_tasks.keys())
    l.debug(f'read_tasks_name={read_tasks_name}')
    assert_equal(sorted(task_names), sorted(read_tasks_name))


@then('we read the help command output')
def check_help_command_output(ctx):
    l = log.getChild(check_help_command_output.__name__)
    lines = ctx.client.readlines_stdout()
    l.debug(f'lines={lines}')
    raise NotImplementedError
