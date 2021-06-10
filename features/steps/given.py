from os import remove
from typing import Any, Callable, Union
from features.steps.lib.server_proc import ServerProc
from features.steps.lib.client_proc import ClientProc
from behave import given
import logging

log = logging.getLogger('given')


def load_config_file(file: str, type: str):
    if type == 'application/yaml':
        import yaml
        with open(file) as f:
            return yaml.load(f, Loader=yaml.Loader)
    elif type == 'application/toml':
        import toml
        with open(file) as f:
            return toml.load(f)


@given("the config file {file}")
def setup_config_file(ctx, file):
    import mimetypes
    l = log.getChild(setup_config_file.__name__)
    l.debug(f'file={file}')
    ctx.config_file = file
    ctx.config_file_type = mimetypes.guess_type(file)
    ctx.config_file_data = load_config_file(
        ctx.config_file, ctx.config_file_type[0])


@given("the verbose level as {level}")
def setup_verbose_level(ctx, level):
    l = log.getChild(setup_verbose_level.__name__)
    l.debug(f'level={level}')
    ctx.verbose_level = level


@given("the format as \"{format}\"")
def setup_format(ctx, format):
    l = log.getChild(setup_format.__name__)
    l.debug(f'format={format}')
    ctx.format = format


@given("the client is running")
def start_client(ctx):
    ctx.client = ClientProc()


@given("server is running")
def start_server(ctx):
    ctx.server = ServerProc('configs/example.yml', 'debug')
