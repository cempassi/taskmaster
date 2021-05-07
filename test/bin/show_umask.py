#!/usr/bin/env python3
from os import umask

print("previous umask: {:o}".format(umask(0)))
