import random
import time
from typing import Callable

from count import count_approx_py, count_exact_py
from count_rs import count_approx_rs


def timeit(f: Callable, *args, **kwargs):
    start = time.time()
    for _ in range(10):
        f(*args, **kwargs)
    print(f.__name__, (time.time() - start) / 10)


WORDS = [str(i) for i in range(100_000)] * 100
random.shuffle(WORDS)

print("exact", count_exact_py(WORDS))
print("approx", count_approx_py(WORDS))

timeit(count_exact_py, WORDS)
timeit(count_approx_py, WORDS)
timeit(count_approx_rs, WORDS)
