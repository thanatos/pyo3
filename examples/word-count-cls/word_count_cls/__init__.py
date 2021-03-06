# -*- coding: utf-8 -*-
from __future__ import absolute_import

from ._word_count import Words

__all__ = ['Words', 'search_py']


def search_py(path, needle):
    total = 0
    with open(path, 'r') as f:
        for line in f:
            words = line.split(' ')
            for word in words:
                if word == needle:
                    total += 1
    return total
