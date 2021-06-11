
import logging

from behave.fixture import use_fixture_by_tag
from behave.model import Tag
from features.fixture_lib import clean_server, set_logger, remove_tmp_file, setup_mimetypes

handler = logging.FileHandler('environment.log', mode='w')
handler.setFormatter(logging.Formatter(fmt=logging.BASIC_FORMAT))
log = logging.getLogger('environment')
log.setLevel(logging.DEBUG)
log.addHandler(handler)

set_logger(log)

fixture_registry = {
    'fixture.setup_mimetypes': setup_mimetypes,
    'fixture.clean_server': clean_server,
    'fixture.remove_tmp_file': remove_tmp_file,
}


def before_tag(ctx, tag: Tag):
    if tag.startswith('fixture.'):
        use_fixture_by_tag(tag, ctx, fixture_registry)
