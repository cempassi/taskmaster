
import logging

from behave.fixture import fixture, use_fixture
from behave.model import Scenario
from features.steps.lib.taskmaster_utils import TASKMASTER_SOCK

handler = logging.FileHandler('environment.log', mode='w')
handler.setFormatter(logging.Formatter(fmt=logging.BASIC_FORMAT))
log = logging.getLogger('environment')
log.setLevel(logging.DEBUG)
log.addHandler(handler)


@fixture
def clean_server(ctx):
    log.debug('remove previous socket')
    try:
        import os
        os.unlink(TASKMASTER_SOCK)
        log.debug('file successfuly removed')
    except FileNotFoundError:
        log.debug('file not existing, pass')
    yield
    log.debug('stopping server')
    ctx.server.close()


def before_scenario(ctx, scenario: Scenario):
    log.info(f'before_scenario={scenario.name}')
    if 'fixture.clean_server' in scenario.tags:
        use_fixture(clean_server, ctx)
