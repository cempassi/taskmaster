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


@fixture(name="fixture.remove_tmp_file")
def remove_tmp_file(ctx):
    l = log.getChild(remove_tmp_file.__name__)
    l.debug('wait for cleanup')
    yield
    from os import unlink

    l.debug(f'tmp_file={ctx.tmp_file}')
    assert ctx.tmp_file, 'no tmp file to remove'
    unlink(ctx.tmp_file)
    ctx.tmp_file = None


@fixture(name='fixture.setup_mimetypes')
def setup_mimetypes(_ctx):
    l = log.getChild(setup_mimetypes.__name__)
    import mimetypes

    l.debug('setup mimetypes')
    mimetypes.add_type('application/yaml', '.yml')
    mimetypes.add_type('application/yaml', '.yaml')
    mimetypes.add_type('application/toml', '.toml')
