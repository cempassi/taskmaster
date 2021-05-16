from signal import signal, strsignal, Signals
from time import sleep
from sys import argv, exit

STOP_AT_SIGNAL = False


def handler(signum, _frame):
    print("receive signal {}({})".format(strsignal(signum), signum))
    if STOP_AT_SIGNAL:
        exit(signum)


if __name__ == '__main__':
    if len(argv) > 1:
        STOP_AT_SIGNAL = True

    for sig in set(Signals):
        print('setting signal {}({})'.format(strsignal(sig), sig))
        try:
            signal(sig, handler)
        except OSError as e:
            print("failed to set handler for {}, skipping ...".format(
                strsignal(sig)))

    while True:
        print("waiting for signal")
        sleep(200)
