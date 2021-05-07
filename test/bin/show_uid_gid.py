#!/usr/bin/env python3
from os import getuid, getgid, geteuid, getegid

print("current uid: {}".format(getuid()))
print("current gid: {}".format(getgid()))
print("current euid: {}".format(geteuid()))
print("current egid: {}".format(getegid()))
