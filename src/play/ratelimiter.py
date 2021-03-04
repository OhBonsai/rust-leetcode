"""
BASIC: provide a rate limiter make sure function call time less than *max_call* time in *period* second @@
ADVANCED:
    1. what happen if exceed the limit? Just raise exception or sleep ?
    2. if function call will cost some second? calculate from the start of function call or the end?
    3. if exceeding, start a hook call @@
CODE TASTE:
    1. wrapper?  @@
    2. context?
    3. param check  @@
PARALLEL SAFE:
    1. thread safe?
    2. support async await
"""
import time
import threading

from typing import Callable, Optional
from functools import wraps
from collections import deque
from enum import Enum


class EStrategy(Enum):
    Raise: int = 0
    Sleep: int = 1


class CStrategy(Enum):
    Begin: int = 0
    End: int = 1


class RateLimiter:

    def __init__(self, max_call: int, period: int,
                 exceed_hook: Callable[[int, int], None] = None,
                 exceed_strategy: int = EStrategy.Raise,
                 calc_strategy: int = CStrategy.Begin):
        if max_call <= 0:
            raise ValueError("max_call must be positive")

        if period <= 0:
            raise ValueError("period must be positive")

        self.max_call = max_call
        self.period = period
        self.exceed_hook = exceed_hook
        self.exceed_strategy = exceed_strategy
        self.calc_strategy = calc_strategy
        self._calls = deque()
        self._lock = threading.Lock()

    def _is_exceed(self):
        return len(self._calls) >= self.max_call

    def _record_latest(self):
        self._calls.append(time.time())

    def _clean_expired(self):
        while len(self._calls) > 0 and time.time() - self._calls[0] > self.period:
            self._calls.popleft()

    def __enter__(self):
        with self._lock:
            if self._is_exceed():
                if self.exceed_strategy == EStrategy.Sleep:
                    until = self._calls[0] + self.period
                    wait = until - time.time()
                    if self.exceed_hook:
                        hook_thread = threading.Thread(target=self.exceed_hook, args=(wait, until))
                        hook_thread.setDaemon(True)
                        hook_thread.start()
                    if wait > 0:
                        time.sleep(wait)
                else:
                    raise Exception("exceed max_call {} limit in {} second", self.max_call, self.period)

            if self.calc_strategy == CStrategy.Begin:
                self._record_latest()
            return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        with self._lock:
            if self.calc_strategy == CStrategy.End:
                self._record_latest()
            self._clean_expired()

    def __call__(self, func: Callable):
        @wraps(func)
        def inner(*args, **kwargs):
            with self:
                return func(*args, **kwargs)
        return inner


def run_when_exceed(wait: int, until: int):
    print("wait {} until {}", wait, until)


def run_when_exceed_timeout(wait: int, until: int):
    print("threading id: {}".format(threading.get_ident()))
    run_when_exceed(wait, until)
    time.sleep(2)
    print("threading id: {} I am a long time hook".format(threading.get_ident()))


@RateLimiter(max_call=2, period=1, exceed_strategy=EStrategy.Sleep, exceed_hook=run_when_exceed)
def work(*args, **kwargs):
    print(args, kwargs)


def work_timeout(*args, **kwargs):
    print(args, kwargs)
    time.sleep(1)


if __name__ == "__main__":

    for i in range(30):
        t = threading.Thread(target=work, args=(1,2), kwargs={"a": 3})
        t.setDaemon(True)
        t.start()

    for i in range(100):
        work(1, 2, a=3)
