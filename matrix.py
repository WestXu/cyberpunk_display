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

    @property
    def array(self):
        array = np.zeros((8, 32))
        for col, i in enumerate(self.int_ls):
            array[7 - i, col] = 1

        return array

    @property
    def up_down_ls(self):
        up_down_ls = [0]
        for i in range(1, 32):
            p = self.num_ls[i]
            pre_p = self.num_ls[i - 1]
            if p > pre_p:
                up_down_ls.append(1)
            elif p < pre_p:
                up_down_ls.append(-1)
            else:
                up_down_ls.append(0)
        return up_down_ls

    def plot(self):
        black, white = "██", "  "
        print(
            "\n".join(
                [
                    "".join([black if i == 1 else white for i in row])
                    for row in self.array
                ]
            )
        )

    def to_pixel(self):
        array = self.array.copy()
        up_down_ls = self.up_down_ls

        def find_y(x):
            tarray = array[:, x]
            for y, i in enumerate(tarray):
                if i == 1:
                    return y

        return [
            {
                "type": "pixel",
                "position": [x, y],
                "color": (
                    [0, 150, 0]
                    if up_down_ls[x] == 1
                    else [150, 0, 0]
                    if up_down_ls[x] == -1
                    else [0, 0, 150]
                ),
            }
            for x in range(32)
            if (y := find_y(x)) is not None
        ]


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
