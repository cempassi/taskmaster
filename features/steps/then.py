from typing import List
from behave import then, register_type, use_step_matcher
from features.steps.lib.pattern import parse_int
import logging

log = logging.getLogger('then')

register_type(Int=parse_int, String=str)
use_step_matcher('cfparse')


@then('server has read {task_to_read:Int} tasks')
def assert_tasks_read(ctx, task_to_read):
    l = log.getChild(assert_tasks_read.__name__)
    l.debug(f'task_to_read={task_to_read}')
    raise NotImplementedError


@then('server is still running')
def assert_process_is_running(ctx):
    l = log.getChild(assert_process_is_running.__name__)
    isrunning = ctx.server.is_running()
    l.debug(f'isrunning={isrunning}')
    assert isrunning


@then('the tasks are named {task_names:String+}')
def assert_task_names(ctx, task_names: List[str]):
    l = log.getChild(assert_task_names.__name__)
    l.debug(f'task_names={task_names}')
    raise NotImplementedError
