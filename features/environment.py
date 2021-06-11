
import logging

from behave.fixture import use_fixture_by_tag
from behave.model import Tag
from features.fixture_lib import set_logger, fixtures_registry

handler = logging.FileHandler('environment.log', mode='w')
handler.setFormatter(logging.Formatter(fmt=logging.BASIC_FORMAT))
log = logging.getLogger('environment')
log.setLevel(logging.DEBUG)
log.addHandler(handler)

set_logger(log)


def before_tag(ctx, tag: Tag):
    if tag.startswith('fixture.'):
        use_fixture_by_tag(tag, ctx, fixtures_registry)
