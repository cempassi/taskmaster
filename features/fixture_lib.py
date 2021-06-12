from features.steps.lib.server_mock import ServerMock
from features.steps.lib.client_mock import ClientMock
from os import remove
from behave import fixture
import logging

from behave.fixture import fixture
from features.steps.lib.taskmaster_utils import TASKMASTER_SOCK


log = logging.getLogger('fixtures')


def set_logger(new_log):
    global log
    log = new_log


@fixture(name='fixture.clean_server')
def clean_server(ctx):
    l = log.getChild(clean_server.__name__)
    l.debug('remove previous socket')
    try:
        import os
        os.unlink(TASKMASTER_SOCK)
        l.debug('file successfuly removed')
    except FileNotFoundError:
        l.debug('file not existing, pass')
    yield
    l.debug('stopping server')
    ctx.server.close()


@fixture(name="fixture.remove_tmp_files")
def remove_tmp_files(ctx):
    l = log.getChild(remove_tmp_files.__name__)
    l.debug('create empty tmp files')
    ctx.tmp_files = []
    l.debug('waiting for cleanup')
    yield
    from os import unlink

    l.debug(f'tmp_files={ctx.tmp_files}')
    while len(ctx.tmp_files):
        file = ctx.tmp_files.pop()
        l.debug(f'remove file {file}')
        unlink(file)


@fixture(name='fixture.setup_mimetypes')
def setup_mimetypes(_ctx):
    l = log.getChild(setup_mimetypes.__name__)
    import mimetypes

    l.debug('setup mimetypes')
    mimetypes.add_type('application/yaml', '.yml')
    mimetypes.add_type('application/yaml', '.yaml')
    mimetypes.add_type('application/toml', '.toml')


@fixture(name='fixture.use_client_mock')
def setup_client_mock(ctx):
    ctx.client_mock = ClientMock()


@fixture(name='fixture.use_server_mock')
def setup_server_mock(ctx):
    l = log.getChild(setup_server_mock.__name__)
    l.debug('starting server mock')
    ServerMock.log = log.getChild(ServerMock.__name__)
    ctx.server_mock = ServerMock()
    ctx.server_mock.log = log.getChild(ServerMock.__name__)
    yield

    l.debug('stopping server mock')
    if 'server_mock' in ctx:
        del ctx.server_mock
    else:
        l.warn('server mock already stopped/non existing')


fixtures_registry = dict()

for func in setup_mimetypes, remove_tmp_files, clean_server, setup_client_mock, setup_server_mock:
    fixtures_registry[func.name] = func
