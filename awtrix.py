import asyncio
import json
from typing import Literal

import aiohttp
from loguru import logger

from ws_coin import Huobi


async def push(ssn: aiohttp.ClientSession, data: dict, endpoint: str):
    async with ssn.post(
        f'http://localhost:7000/api/v3/{endpoint}',
        data=json.dumps(data),
        headers={'Content-Type': 'application/json'},
    ) as res:
        return res.text


async def draw_exit(ssn: aiohttp.ClientSession):
    await push(
        ssn,
        {
            "draw": [{"type": "exit"}],
        },
        endpoint='draw',
    )


async def draw_price(ssn: aiohttp.ClientSession, price):
    str_price = str(price)
    await push(
        ssn,
        {
            "draw": [
                {"type": "fill", "color": [50, 50, 50]},
                {
                    "type": "text",
                    "string": str_price,
                    "position": [1, 1],
                    "color": [255, 255, 255],
                },
                {"type": "show"},
            ],
        },
        endpoint='draw',
    )


class Awtrix:
    def __init__(self, loop) -> None:
        self._ssn = aiohttp.ClientSession()
        self._q: asyncio.Queue = asyncio.Queue(maxsize=1)

        self.loop = loop

    async def update(self, p):
        if not self._q.empty():
            await self._q.get()

        await self._q.put(p)

    async def send(self, p):
        await draw_price(self._ssn, p)
        logger.info(f'Sent to awtrix')

    async def send_latest(self):
        p = await self._q.get()
        await self.send(p)


async def data(awtrix: Awtrix):
    hb = Huobi(markets=['btcusdt'])
    await hb._connect()

    while True:
        market, p = await hb.recv_price()
        logger.info(f"{market} {p}")
        await awtrix.update(p)


async def pushing(awtrix: Awtrix):
    while True:
        await awtrix.send_latest()


if __name__ == "__main__":

    loop = asyncio.get_event_loop()
    awtrix = Awtrix(loop)

    loop.create_task(data(awtrix))
    loop.run_until_complete(pushing(awtrix))
