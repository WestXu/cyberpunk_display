import asyncio

import numpy as np


class PriceQueue:
    def __init__(self):
        self.q = asyncio.Queue(maxsize=32)
        self.lock = asyncio.Lock()

    async def update(self, p):
        async with self.lock:
            if self.q.full():
                await self.q.get()
            await self.q.put(p)

    async def tolist(self):
        async with self.lock:
            assert not self.q.empty()
            l = list(self.q._queue)
        if len(l) == 32:
            return l
        return [l[0]] * (32 - len(l)) + l


class Matrix:
    def __init__(self, num_ls: list):
        assert len(num_ls) == 32
        self.num_ls = num_ls
        self.sr = np.array(num_ls)

    @property
    def int_ls(self):
        sr = self.sr
        rg = sr.max() - sr.min()
        if rg == 0:
            return [3] * 32
        return ((sr - sr.min()) / rg * 6).round().astype(int).tolist()

    def plot(self):
        matrix = np.zeros((7, 32))
        for col, i in enumerate(self.int_ls):
            matrix[6 - i][col] = 1

        black, white = "██", "  "
        print(
            "\n".join(
                ["".join([black if i == 1 else white for i in row]) for row in matrix]
            )
        )


if __name__ == "__main__":
    import random

    async def main():
        p = 0
        pq = PriceQueue()
        while True:
            p += random.randint(-10, 10)
            await pq.update(p)
            Matrix(await pq.tolist()).plot()
            await asyncio.sleep(0.1)

    asyncio.run(main())
