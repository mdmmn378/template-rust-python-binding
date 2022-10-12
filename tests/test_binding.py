import rustify
import numpy as np
import time

arr = np.random.randint(0, 2 ^ 30, 1000000)


def test_sort_time():
    start = time.time()
    from_rust = rustify.sort(arr)
    end = time.time()
    print("rust taken ", end - start)

    start = time.time()
    arr.sort()
    end = time.time()
    print("python taken ", end - start)

    assert (from_rust == arr).all()
