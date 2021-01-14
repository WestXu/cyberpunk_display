import asyncio
from functools import cached_property

import numpy as np


def rgb888_to_rgb565(im: np.array) -> np.array:
    """copy from https://stackoverflow.com/a/61521134
    >>> import numpy as np
    >>> np.random.seed(0)
    >>> im = np.random.randint(0, 256, (1, 4, 3), dtype=np.uint8)
    >>> im
    array([[[172,  10, 127],
            [140,  47, 170],
            [196, 151, 117],
            [166,  22, 183]]], dtype=uint8)
    >>> rgb888_to_rgb565(im)
    array([[43087, 35189, 50350, 41142]], dtype=uint16)
    """
    R5 = (im[..., 0] >> 3).astype(np.uint16) << 11
    G6 = (im[..., 1] >> 2).astype(np.uint16) << 5
    B5 = (im[..., 2] >> 3).astype(np.uint16)

    RGB565 = R5 | G6 | B5

    return RGB565


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
        self.num_ls = list(num_ls)
        self.sr = np.array(num_ls)

    @cached_property
    def int_ls(self):
        sr = self.sr
        rg = sr.max() - sr.min()
        if rg == 0:
            return [3] * 32
        return ((sr - sr.min()) / rg * 6).round().astype(int).tolist()

    @cached_property
    def array(self):
        '''二维，1表示有点，0表示没有点'''
        array = np.zeros((8, 32))
        for col, i in enumerate(self.int_ls):
            array[7 - i, col] = 1

        return array

    @cached_property
    def up_down_ls(self):
        '''1维列表。-1跌，0平，1涨。第一个值永远为0'''
        diff = np.diff(self.sr)
        diff[diff > 0] = 1
        diff[diff < 0] = -1
        return [0] + diff.tolist()

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

    def to_pixel(self) -> np.array:
        up_down_matrix = (
            np.tile(
                np.array(self.up_down_ls) + 2,  # 1跌2平3涨
                (8, 1),  # 1行复制为8行，这样每列的数字都相同
            )
            * self.array  # 每列只保留有效点
        )
        R = np.where(up_down_matrix == 1, 255, 0)
        G = np.where(up_down_matrix == 2, 255, 0)
        B = np.where(up_down_matrix == 3, 255, 0)
        RGB = np.concatenate(
            [R[:, :, None], G[:, :, None], B[:, :, None]], axis=2
        )  # 把R/G/B三个2维色相拼成RGB图像
        return rgb888_to_rgb565(RGB)  # 转化为RGB565


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
