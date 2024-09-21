import math
import random
from typing import Sequence


def count_exact_py(items: Sequence) -> int:
    return len(set(items))


def count_approx_py(items: Sequence, epsilon=0.5, delta=0.001) -> int:
    p: float = 1.0
    tracked_items = set()
    max_tracked = round(((12 / (epsilon**2)) * math.log2(8 * len(items) / delta)))

    for item in items:
        tracked_items.discard(item)
        if random.random() < p:
            tracked_items.add(item)
        if len(tracked_items) == max_tracked:
            # drop tracked values with coin toss
            tracked_items = {item for item in tracked_items if random.random() < 0.5}
            p /= 2
            if len(tracked_items) == 0:
                raise RuntimeError("unlucky")
    return int(round(len(tracked_items) / p))
