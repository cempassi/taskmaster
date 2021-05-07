from os import remove
from features.steps.lib.server_proc import ServerProc
from features.steps.lib.client_proc import ClientProc
from behave import given
import logging

log = logging.getLogger('given')


@given("the config file {file} in {lang}")
def setup_config_file(ctx, file, lang):
    l = log.getChild(setup_config_file.__name__)
    l.debug(f'file={file}, lang={lang}')
    ctx.config_file = file
    ctx.config_file_type = lang


@given("the verbose level as {level}")
def setup_verbose_level(ctx, level):
    l = log.getChild(setup_verbose_level.__name__)
    l.debug(f'level={level}')
    ctx.verbose_level = level


@given("the client is running")
def start_client(ctx):
    ctx.client = ClientProc()


@given("we remove previous unix socket")
def remove_previous_unix(ctx):
    import os
    from features.steps.lib.taskmaster_utils import TASKMASTER_SOCK

    l = log.getChild(remove_previous_unix.__name__)
    try:
        os.unlink(TASKMASTER_SOCK)
        l.debug(f'{TASKMASTER_SOCK} deleted')
    except FileNotFoundError:
        l.debug(f'{TASKMASTER_SOCK} not exising, skipping')


@given("server is running")
def start_server(ctx):
    ctx.server = ServerProc('configs/example.yml', 'debug')
