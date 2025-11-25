#!/usr/bin/python3
import sys
import time
from math import sqrt as _sqrt
from typing import Callable
# from numba import njit

# @njit
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n == 2:
        return True

    mod = n % 2
    if mod == 0:
        return False

    r = int(_sqrt(n))
    for i in range(3, r + 1, 2):
        if n % i == 0:
            return False
    return True


def gen_primes(gen: int, callback: Callable[[int, int, int], None]):
    counter = 0
    n = 0

    is_prime_local = is_prime
    callback_local = callback
    time_ns = time.perf_counter_ns

    while counter <= gen:
        if is_prime_local(n):
            counter += 1
            callback_local(counter, n, time_ns())
        n += 1


def main(cum: int, width: int):
    start = time.perf_counter_ns()
    with open("./benchmark_python", "w") as file:

        write = file.write
        modulo = width

        def prime_callback(counter: int, prime_val: int, end_time: int):
            if counter % modulo == 0:
                write(f"{counter},{prime_val},{end_time - start}\n")

        gen_primes(cum, prime_callback)
        file.flush()


if __name__ == "__main__":
    main(int(sys.argv[1]), int(sys.argv[2]))
