import json
import time

import aiohttp
from loguru import logger

from .cyberpunk_display import PriceQueueRust, WsCoinRust


class Awtrix:
    def __init__(self, ip='localhost', port=7000, min_interval=0.1) -> None:
        self._ip = ip
        self._port = port

        self._min_interval = min_interval

        self._last_sent_time = 0

    async def __aenter__(self):
        self._ssn = aiohttp.ClientSession()
        return self

    async def __aexit__(self, exc_type, exc, tb):
        await self._draw_exit()
        await self._ssn.close()

    async def _draw_exit(self):
        await self._push(
            {
                "draw": [{"type": "exit"}],
            },
            endpoint='draw',
        )

    async def _push(self, data: dict, endpoint: str):
        async with self._ssn.post(
            f'http://{self._ip}:{self._port}/api/v3/{endpoint}',
            data=json.dumps(data),
            headers={'Content-Type': 'application/json'},
        ) as res:
            return res.text

    async def plot(self, pq):
        if time.time() - self._last_sent_time < 0.1:
            '''小于0.1秒的间隔没有必要发送，人眼无法分辨'''
            logger.info('Skipped sending because of too little interval.')
            return

        await self._push(
            {
                "draw": (
                    [
                        {
                            "type": "bmp",
                            "position": [0, 0],
                            "size": [32, 8],
                            "data": pq.to_rgb565(),
                        },
                        {"type": "show"},
                    ]
                ),
            },
            endpoint='draw',
        )
        self._last_sent_time = time.time()


async def main(*args, **kwargs):
    async with Awtrix(*args, **kwargs) as awtrix:
        pq = PriceQueueRust()

        print("\n\n\n\n\n\n\n\n")
        for p in WsCoinRust():
            pq.push(p)
            print(f"\x1b[8A{pq.to_plot()}")
            await awtrix.plot(pq)
