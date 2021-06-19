from features.steps.lib.utils import load_config_file
from features.steps.lib.client_proc import ClientProc
from behave import when
from features.steps.lib.server_proc import ServerProc
import logging

log = logging.getLogger('when')


@when('the server is running')
def run_server(ctx):
    l = log.getChild(run_server.__name__)
    ctx.server = ServerProc(config=ctx.config_file,
                            verbose=ctx.verbose_level, format=ctx.format, logfile=ctx.logfile)
    l.debug(f'server={ctx.server!s}')


@when('the client is running')
def run_client(ctx):
    l = log.getChild(run_client.__name__)
    ctx.client = ClientProc(verbose=ctx.verbose_level,
                            logfile=ctx.logfile if 'logfile' in ctx else None)
    l.debug(f'client={ctx.client}')


@when('we write {command:String}')
def write_client_command(ctx, command: str):
    l = log.getChild(write_client_command.__name__)
    l.debug(f'command={command}')
    ctx.client.write(command)


@when('we skip current output')
def flush_stdout(ctx):
    l = log.getChild(flush_stdout.__name__)
    l.debug('flush client stdout')
    ctx.client.flush_out()


@when('we add the following to the current config file')
def edit_current_config_file(ctx):
    l = log.getChild(edit_current_config_file.__name__)
    l.debug(f'current_file={ctx.config_file} text={ctx.text}')
    with open(ctx.config_file, 'a') as f:
        f.write('\n')
        f.write(ctx.text)
    ctx.execute_steps(f'Given the config file {ctx.config_file}')


@when('we sleep for {time:Float}')
def sleep_for(ctx, time):
    l = log.getChild(sleep_for.__name__)
    l.debug(f'time={time}')
    from time import sleep

    sleep(time)


@when('we stop the server mock')
def stop_server_mock(ctx):
    l = log.getChild(stop_server_mock.__name__)
    del ctx.server_mock


@when('the client send the command "{command}"')
def send_command(ctx, command):
    l = log.getChild(send_command.__name__)
    l.debug(f'command={command}')
    ctx.client.write(f'{command}\n')
    ctx.client.flush_in()


@when('we edit the current config file with')
def edit_current_config_file(ctx):
    l = log.getChild(edit_current_config_file.__name__)
    assert ctx.text is not None, 'we need a text to edit the config with'
    assert 'config_file' in ctx, 'we need an existing config file loaded'
    l.info(f'current_config={ctx.config_file}, data_size={len(ctx.text)}')
    l.debug(f'text={ctx.text}')

    with open(ctx.config_file, 'w') as f:
        f.write(ctx.text)

    ctx.config_data = load_config_file(ctx.config_file, ctx.config_type[0])
