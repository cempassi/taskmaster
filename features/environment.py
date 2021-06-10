
import logging

from behave.fixture import fixture, use_fixture
from behave.model import Feature, Scenario
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


@fixture
def setup_mimetypes(ctx):
    import mimetypes

    log.debug('setup mimetypes')
    mimetypes.add_type('application/yaml', '.yml')
    mimetypes.add_type('application/yaml', '.yaml')
    mimetypes.add_type('application/toml', '.toml')


def before_scenario(ctx, scenario: Scenario):
    log.info(f'before_scenario={scenario.name}')
    if 'fixture.clean_server' in scenario.tags:
        use_fixture(clean_server, ctx)


def before_feature(ctx, feature: Feature):
    log.info(f'before_feature={feature.name}')
    if 'fixture.setup_mimetypes' in feature.tags:
        use_fixture(setup_mimetypes, ctx)
