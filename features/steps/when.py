from features.steps.lib.client_mock import ClientMock
from os import SEEK_END
from types import SimpleNamespace
from behave import fixture, when
from subprocess import Popen, PIPE
from features.steps.lib.server_proc import ServerProc
import logging

log = logging.getLogger('when')


@when('the server is running')
def run_server(ctx):
    l = log.getChild(run_server.__name__)
    ctx.server = ServerProc(config=ctx.config_file,
                            verbose=ctx.verbose_level, format=ctx.format, logfile=ctx.logfile)
    l.debug(f'server={ctx.server!s}')


@when('we write {command:String}')
def write_client_command(ctx, command: str):
    l = log.getChild(write_client_command.__name__)
    l.debug(f'command={command}')
    ctx.client.write(command)


@when('we skip current output')
def flush_stdout(ctx):
    l = log.getChild(flush_stdout.__name__)
    l.debug('flush client stdout')
    ctx.client.flush_stdout()


@when('we ask for tasks')
def list_tasks(ctx):
    assert ctx.server.is_running(), 'server is not running'
    mock = ClientMock()
    res = mock.send_list()
    assert res['type'] == 'tasks', 'check the return type'
    ctx.read_tasks = res['tasks']
