import asyncio
import json
from typing import Literal

import aiohttp
from loguru import logger

from matrix import Matrix, PriceQueue
from ws_coin import Huobi


class Awtrix:
    def __init__(self) -> None:
        self._q: asyncio.Queue = asyncio.Queue(maxsize=1)
        self._pq = PriceQueue()

        self._last_sent = 0

    async def __aenter__(self):
        self._ssn = aiohttp.ClientSession()
        return self

    async def __aexit__(self, exc_type, exc, tb):
        await self.draw_exit()
        await self._ssn.close()

    async def _push(self, data: dict, endpoint: str):
        async with self._ssn.post(
            f'http://localhost:7000/api/v3/{endpoint}',
            data=json.dumps(data),
            headers={'Content-Type': 'application/json'},
        ) as res:
            return res.text

    async def draw_exit(self):
        await self._push(
            {
                "draw": [{"type": "exit"}],
            },
            endpoint='draw',
        )

    async def plot_price(self, p):
        await self._push(
            {
                "draw": (
                    [{"type": "fill", "color": [50, 50, 50]}]
                    + Matrix(await self._pq.tolist()).to_pixel()
                    + [
                        {
                            "type": "text",
                            "string": f"{p:.2f}",
                            "position": [1, 1],
                            "color": [255, 255, 255],
                        },
                        {"type": "show"},
                    ]
                ),
            },
            endpoint='draw',
        )

    async def draw_price(self, price, pre_p):
        trend: int = round(price - pre_p)
        anchor_x = 16
        end_x = anchor_x + trend
        end_x = min(max(end_x, 0), 31)

        await self._push(
            {
                "draw": [
                    {"type": "fill", "color": [50, 50, 50]},
                    {
                        "type": "text",
                        "string": f"{price:.2f}",
                        "position": [1, 1],
                        "color": [255, 255, 255],
                    },
                    {
                        "type": "line",
                        "start": [anchor_x, 7],
                        "end": [end_x, 7],
                        "color": [
                            255 if trend < 0 else 100,
                            255 if trend > 0 else 100,
                            100,
                        ],
                    },
                    {"type": "show"},
                ],
            },
            endpoint='draw',
        )

    async def update(self, p):
        if not self._q.empty():
            await self._q.get()
        await self._q.put(p)

        await self._pq.update(p)

    async def send(self, p, pre_p):
        await self.plot_price(p)
        logger.info(f'Sent to awtrix')

    async def send_latest(self):
        p = await self._q.get()
        await self.send(p, pre_p=self._last_sent)
        self._last_sent = p


async def data(awtrix: Awtrix):
    hb = Huobi(markets=['btcusdt'])
    await hb._connect()

    while True:
        market, p = await hb.recv_price()
        logger.info(f"{market} {p}")
        await awtrix.update(p)


async def push(awtrix: Awtrix):
    while True:
        await awtrix.send_latest()


if __name__ == "__main__":

    async def main(loop):
        async with Awtrix() as awtrix:
            loop.create_task(data(awtrix))
            await push(awtrix)

    loop = asyncio.get_event_loop()
    loop.run_until_complete(main(loop))
