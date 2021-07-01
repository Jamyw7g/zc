#!/usr/bin/env python3
#
# Copyright(C) 2021 wuyaoping
#

import os
import os.path as osp
import shutil

SHELLS = ('zsh', 'bash', 'fish')


def get_shell():
    shell = osp.basename(os.getenv('SHELL'))
    assert shell and shell in SHELLS, "just support {}".format(SHELLS)

    if shell == 'sh':
        shell = 'bash'
    return shell


def main():
    config_path = osp.expanduser('~/.zc/share')
    if not osp.exists(config_path):
        os.makedirs(config_path)
    shell = get_shell()

    shutil.copy("zc.{}".format(shell), config_path)

    if shell != 'fish':
        rcfile = osp.expanduser('~/.{}rc'.format(shell))
    else:
        rcfile = osp.expanduser('~/config/fish/config.fish')
    activate = 'source {}'.format(osp.join(config_path, 'zc.{}'.format(shell)))

    if os.system('echo "{}" >> {}'.format(activate, rcfile)) == 0:
        print('run command `{}` to activate environment'.format(activate))


if __name__ == '__main__':
    main()
