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


@when('we write {command:String}')
def write_client_command(ctx, command: str):
    l = log.getChild(write_client_command.__name__)
    l.debug(f'command={command}')
    ctx.client.write(command)


@when('we skip current output')
def flush_stdout(ctx):
    l = log.getChild(flush_stdout.__name__)
    l.debug('flush client stdout')
    ctx.client.flush_stdout()


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
