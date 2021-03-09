import pytest
from functools import wraps
from typing import Callable
from collections import namedtuple
import time


class Node:
    __slots__ = ['pre', 'after', 'key', 'result', 'ts']

    def __init__(self, pre, after, key, result, ts):
        self.pre = pre
        self.after = after
        self.key = key
        self.result = result
        self.ts = ts

    def __iter__(self):
        return [self.pre, self.after, self.key, self.result, self.ts].__iter__()


# 又可以抛出无法被hash的异常， 如果一个参数是无法被hash，需要考虑是否适合缓存
def _make_key(*args, **kwargs):
    key_tuple = tuple(args)
    if kwargs:
        for x in kwargs.items():
            key_tuple += x
    return hash(key_tuple)


class Cache:

    def __init__(self,
                 max_size=100,
                 expire_sec=30,
                 expire_hook: Callable[[str], None] = None,
                 hit_hook: Callable[[str], None] = None,
                 not_hit_hook: Callable[[str], None] = None,
                 ):
        self.max_size = max_size
        self.expire_sec = expire_sec

        self.expire_hook = expire_hook
        self.hit_hook = hit_hook
        self.not_hit_hook = not_hit_hook

        self._cache = {}
        self._full = False
        self._header = Node(None, None, None, None, 0)
        self._header.pre = self._header
        self._header.after = self._header

    def __call__(self, fn):
        @wraps(fn)
        def inner(*args, **kwargs):
            key = _make_key(*args, **kwargs)

            result = self.get(key)
            if not result:
                result = fn(*args, **kwargs)
                self.set(key, result)
            return result
        return inner

    def get(self, key):
        if key in self._cache:
            pre, after, key, result, ts = self._cache[key]
            if time.time() - ts > self.expire_sec:
                if self.expire_hook:  self.expire_hook(key)
                return None

            # 将cache放到最新
            pre.after = after
            after.pre = pre

            last = self._header.pre
            last.after = self._cache[key]
            self._cache[key].pre = last
            self._header.pre = self._cache[key]
            self._cache[key].ts = time.time()

            if self.hit_hook:  self.hit_hook(key)
            return result

        if self.not_hit_hook: self.not_hit_hook(key)
        return None

    def set(self, key, result):
        if self._full:
            # 将最老的替换掉
            # 直接将新数据放到头节点，并一定头节点指针
            oldest_key = self._header.key
            self._header.key = key
            self._header.result = result
            self._header.ts = time.time()

            del self._cache[oldest_key]
            self._cache[key] = self._header
            self._header = self._header.after
        else:
            if self._header.key is None:
                # 从来没有初始化
                self._header.key = key
                self._header.result = result
                self._header.ts = time.time()
                self._cache[key] = self._header
            else:
                # 加到最新
                last = self._header.pre
                self._cache[key] = Node(last, self._header, key, result, time.time())
                last.after = self._cache[key]
                self._header.pre = self._cache[key]
            self._full = len(self._cache) >= self.max_size


def test_simple_cache():
    call_time = []

    @Cache()
    def work(*args, **kwargs):
        call_time.append(1)
        return 1

    work(1, 2)
    work(1, 2)
    work(1, a=1)
    work(1, a=2)
    work(1, b=1)
    work(1, a=1)

    assert len(call_time) == 4


def test_expire_cache():
    call_time = []

    @Cache(expire_sec=2)
    def work(*args, **kwargs):
        call_time.append(1)
        return 1

    work(1, 2)
    work(1, a=1)
    work(1, a=2)
    work(1, b=1)
    work(1, a=1)

    time.sleep(3)
    work(1, 2)
    assert len(call_time) == 5


def test_hook():
    not_hit = []
    hit = []
    expire = []

    def not_hit_hook(key): not_hit.append(1)
    def hit_hook(key): hit.append(1)
    def expire_hook(key): expire.append(1)

    @Cache(expire_sec=1, not_hit_hook=not_hit_hook, hit_hook=hit_hook, expire_hook=expire_hook)
    def work(*args, **kwargs):
        return 1

    work(1, 2)
    work(1, a=1)
    work(1, a=2)
    work(1, b=1)
    work(1, a=1)
    time.sleep(2)
    work(1, 2)

    assert len(hit) == 1
    assert len(expire) == 1
    assert len(not_hit) == 4


def test_if_full():

    call_time = []

    @Cache(expire_sec=2, max_size=3)
    def work(*args, **kwargs):
        call_time.append(1)
        return 1

    # lru
    work(1)
    work(2)
    work(3)
    work(4)
    assert len(call_time) == 4

    # cache should has [2,3,4]
    work(1)
    assert len(call_time) == 5

    # cache should hash [3,4, 1]
    work(4)
    assert len(call_time) == 5


if __name__ == "__main__":
    pytest.main(["./lru.py::test_if_full", "-v", "-s"])