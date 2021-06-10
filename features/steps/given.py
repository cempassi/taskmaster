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


@given("server is running")
def start_server(ctx):
    ctx.server = ServerProc('configs/example.yml', 'debug')
