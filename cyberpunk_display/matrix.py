import asyncio

from loguru import logger

from .cyberpunk_display import PriceQueueRust
from .ws_coin import Huobi


class Matrix:
    def __init__(self) -> None:
        self._q: asyncio.Queue = asyncio.Queue(maxsize=1)
        self._pq = PriceQueueRust()

        self._printed = False

    async def update(self, p):
        if not self._q.empty():
            await self._q.get()
        await self._q.put(p)

        self._pq.push(p)

    def _plot(self):
        print('\x1b[8A' * self._printed + self._pq.to_plot())
        self._printed = True

    async def plot_latest(self):
        await self._q.get()
        self._plot()

    async def _data_loop(self):
        hb = Huobi(markets=['btcusdt'])
        await hb._connect()

        while True:
            market, p = await hb.recv_price()
            logger.info(f"{market} {p}")
            await self.update(p)

    async def _plot_loop(self):
        while True:
            await self.plot_latest()

    async def run(self):
        asyncio.get_running_loop().create_task(self._data_loop())
        await self._plot_loop()


async def main():
    await Matrix().run()
