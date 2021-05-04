from types import SimpleNamespace
from behave import fixture, when
from subprocess import Popen, PIPE
from features.steps.lib.server_proc import ServerProc
import logging

log = logging.getLogger('when')

TASKMASTER_PATH = 'target/release/taskmaster'


@fixture
def after_feature(ctx):
    log.warn('calling after feature')


def get_args(ctx: SimpleNamespace):
    l = log.getChild(get_args.__name__)
    args = ['taskmaster']
    if ctx.verbose_level is not None:
        args.extend(['--verbose', ctx.verbose_level])
    if ctx.subcommand is not None:
        args.extend(ctx.subcommand)
    if ctx.config_file is not None:
        args.extend(['--config', ctx.config_file])
    return args


@when('server is running')
def run_server(ctx):
    l = log.getChild(run_server.__name__)
    ctx.subcommand = ['server']
    args = get_args(ctx)
    l.debug(f'args={args}')
    popen = Popen(args, executable=TASKMASTER_PATH, stdout=2, stderr=2)
    ctx.server = ServerProc(popen)
