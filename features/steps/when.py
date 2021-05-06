from types import SimpleNamespace
from behave import fixture, when
from subprocess import Popen, PIPE
from features.steps.lib.server_proc import ServerProc
import logging

log = logging.getLogger('when')


@fixture
def after_feature(ctx):
    log.warn('calling after feature')


@when('server is running')
def run_server(ctx):
    l = log.getChild(run_server.__name__)
    ctx.server = ServerProc(config=ctx.config_file, verbose=ctx.verbose_level)


@when('we write {command:String}')
def write_client_command(ctx, command: str):
    l = log.getChild(write_client_command.__name__)
    l.debug(f'command={command}')
    raise NotImplemented
