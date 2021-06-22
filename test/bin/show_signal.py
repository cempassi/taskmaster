from signal import signal, SIGINT
from time import sleep


def handler(signum, _frame):
    print(f'signaled with {signum}')


signal(SIGINT, handler)

while True:
    print('waiting for signal ...')
    sleep(2)
