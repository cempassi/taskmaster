from features.steps.lib.client_mock import ClientMock
from behave import when
import logging

log = logging.getLogger('client_command')


@when('we ask for tasks')
def list_tasks(ctx):
    assert ctx.server.is_running(), 'server is not running'
    res = ctx.client_mock.send_list()
    assert res['type'] == 'tasks', 'check the return type'
    ctx.read_tasks = res['tasks']


@when('we ask to start \"{taskname}\"')
def start_task(ctx, taskname):
    l = log.getChild(start_task.__name__)
    l.debug(f'taskname={taskname}')
    raise NotImplementedError()


@when('we ask the status of \"{taskname}\"')
def status_task(ctx, taskname):
    l = log.getChild(status_task.__name__)
    l.debug(f'taskname={taskname}')
    raise NotImplementedError()


@when('we ask to stop \"{taskname}\"')
def stop_task(ctx, taskname):
    l = log.getChild(stop_task.__name__)
    l.debug(f'taskname={taskname}')
    raise NotImplementedError


@when('we ask the info about \"{taskname}\"')
def info_task(ctx, taskname):
    l = log.getChild(info_task.__name__)
    l.debug(f'taskname={taskname}')
    raise NotImplementedError


@when('we ask to reload the config')
def reload_config(ctx):
    l = log.getChild(reload_config.__name__)
    raise NotImplementedError


@when('we ask to stop the server')
def stop_server(ctx):
    l = log.getChild(stop_server.__name__)
    raise NotImplementedError
