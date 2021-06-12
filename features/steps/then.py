from typing import List
from behave import then, register_type, use_step_matcher
from features.steps.assert_utils import assert_task, assert_tasks
from features.steps.lib.pattern import parse_float, parse_int
import logging
from asserts import assert_dict_equal, assert_equal, assert_in, assert_true

log = logging.getLogger('then')

register_type(Int=parse_int, String=str, Float=parse_float)
use_step_matcher('cfparse')


@then('the server has read the tasks')
def assert_tasks_read(ctx):
    l = log.getChild(assert_tasks_read.__name__)
    l.debug(f'tasks={ctx.read_tasks}')
    assert_tasks(ctx.read_tasks, ctx.config_data)


@then('the server is still running')
def assert_server_running(ctx):
    l = log.getChild(assert_server_running.__name__)
    isrunning = ctx.server.is_running()
    l.debug(f'isrunning={isrunning}')
    assert isrunning


@then('the client is still running')
def assert_client_running(ctx):
    l = log.getChild(assert_client_running.__name__)
    stderr_lines = ctx.client.readlines_stderr()
    l.debug(f'err_lines={stderr_lines}')
    assert_true(ctx.client.is_running(),
                msg_fmt='client is not running: {msg}')


@then('we read the help command output')
def check_help_command_output(ctx):
    l = log.getChild(check_help_command_output.__name__)
    lines = ctx.client.readlines_stdout()
    l.debug(f'lines={lines}')
    raise NotImplementedError


@then('the status of \"{taskname}\" is \"{status}\"')
def check_task_status(ctx, taskname, status):
    assert 'task_status' in ctx, 'missing task status, did you ask for status to the server ?'
    l = log.getChild(check_task_status.__name__)
    l.debug(f'taskname={taskname}, status={status}')
    l.debug(f'registred_status={ctx.task_status}')

    assert_true(taskname in ctx.task_status,
                msg_fmt=f'missing {taskname} in registred status')
    assert_true(len(ctx.task_status[taskname]),
                msg_fmt=f'registred status is empty')
    assert_equal(ctx.task_status[taskname].pop(0), status)


@then('the status of \"{taskname}\" is one of')
def check_task_status_choice(ctx, taskname):
    assert ctx.table is not None, 'no table given'
    l = log.getChild(check_task_status_choice.__name__)
    choices = list(map(lambda cell: cell['status'], ctx.table))
    l.debug(f'taskname={taskname}, status_choice={choices}')
    l.debug(f'registred_status={ctx.task_status}')

    assert_in(taskname, ctx.task_status,
              msg_fmt=f'missing {taskname} in registred status')
    assert_true(len(ctx.task_status[taskname]),
                msg_fmt=f'registred status is empty')
    got_status = ctx.task_status[taskname].pop(0)

    assert_in(True, list(map(lambda st: st == got_status, choices)),
              msg_fmt=f'{got_status} not in {choices}')


@then('the server sent the info about \"{taskname}\"')
def check_task_info(ctx, taskname):
    assert 'task_info' in ctx, 'missing task info, did you ask for info to the server ?'
    l = log.getChild(check_task_info.__name__)
    l.debug(f'taskname={taskname}')

    assert_in(taskname, ctx.task_info,
              msg_fmt=f'missing {taskname} in registred info')
    task = ctx.task_info[taskname]
    assert_task(task, ctx.config_data[taskname])


@then('the server is stopped')
def check_server_stop(ctx):
    l = log.getChild(check_server_stop.__name__)
    try:
        assert ctx.server.is_running() is False, 'server is not stopped'
    except AssertionError:  # we give Him some time
        import time
        time.sleep(0.2)
        assert ctx.server.is_running() is False, 'server is not stopped'


@then('we have {count:Int} files with the pattern \"{file_pattern}\" containing')
def check_file_created(ctx, count, file_pattern):
    assert ctx.text is not None, "missing text to check content"
    l = log.getChild(check_file_created.__name__)
    l.debug(f'count={count} file_pattern={file_pattern} text="{ctx.text}"')

    from glob import glob
    matches = glob(file_pattern)
    l.debug(f'matches={matches}')

    assert_equal(len(matches), count)
    for filename in matches:
        with open(filename) as f:
            assert_equal(f.read(), ctx.text, msg_fmt=filename +
                         ' not containing expected data: {msg}')
