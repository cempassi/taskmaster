
import logging

from behave.fixture import fixture, use_fixture
from behave.model import Scenario

handler = logging.FileHandler('environment.log')
handler.setFormatter(logging.Formatter(fmt=logging.BASIC_FORMAT))
log = logging.getLogger('environment')
log.setLevel(logging.DEBUG)
log.addHandler(handler)

print('hello')


@ fixture
def clean_server(ctx):
    log.debug('stopping server')
    ctx.server.close()


def after_scenario(ctx, scenario: Scenario):
    log.error(f'after_scenario={scenario.name}')
    if 'fixture.clean_server' in scenario.tags:
        use_fixture(clean_server, ctx)
