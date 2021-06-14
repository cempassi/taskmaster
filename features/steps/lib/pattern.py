from parse import with_pattern
import logging

log = logging.getLogger('pattern')


@with_pattern(r'\d+')
def parse_int(text):
    return int(text)


@with_pattern(r'\d+.\d+')
def parse_float(text):
    return float(text)
