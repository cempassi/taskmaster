from features.steps.lib.utils import load_config_file
from features.steps.lib.server_proc import ServerProc
from features.steps.lib.client_proc import ClientProc
from behave import given
import logging

log = logging.getLogger('given')


@given("the config file {file}")
def setup_config_file(ctx, file):
    import mimetypes
    l = log.getChild(setup_config_file.__name__)
    l.debug(f'file={file}')
    ctx.config_file = file
    ctx.config_type = mimetypes.guess_type(file)
    ctx.config_data = load_config_file(
        ctx.config_file, ctx.config_type[0])


@given("the config in \"{mime}\"")
def setup_config(ctx, mime):
    from tempfile import NamedTemporaryFile
    from mimetypes import guess_extension
    l = log.getChild(setup_config.__name__)
    l.debug(f'mime={mime} text={ctx.text}')
    assert ctx.text is not None, "empty text"

    ext = guess_extension(mime)
    assert ext is not None, f"unknow mime {mime}"

    l.debug(f'create tmp file with suffix {ext}')
    tmp = NamedTemporaryFile('w', suffix=ext, delete=False)
    tmp.write(ctx.text)
    tmp.close()

    filename = tmp.name
    ctx.tmp_files.append(filename)

    l.debug(f'tmp_file={filename}')
    ctx.execute_steps(f'Given the config file {filename}')


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


@given("the log file as \"{filename}\"")
def setup_log_file(ctx, filename):
    l = log.getChild(setup_log_file.__name__)
    l.debug(f'logfile={filename}')
    ctx.logfile = filename


@given("the client is running")
def start_client(ctx):
    ctx.client = ClientProc()


@given("the server is running")
def start_server(ctx):
    ctx.server = ServerProc('configs/example.yml', 'debug')
