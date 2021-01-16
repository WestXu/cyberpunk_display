import asyncio
import gzip
import json
from typing import List, Optional

import websockets
from loguru import logger


class Huobi:
    uri = "wss://api.hadax.com/ws"

    def __init__(self, markets: List[str]) -> None:
        self.markets = markets

    @staticmethod
    def _decode(msg: bytes) -> dict:
        return json.loads(gzip.decompress(msg))

    async def _send(self, data: dict):
        try:
            await self.websocket.send(json.dumps(data))
        except websockets.ConnectionClosedError:
            logger.warning(
                f'ConnectionClosedError sending data {data}, reconnecting...'
            )
            await self._connect()

    async def _recv(self, timeout=None) -> dict:
        async def _retry():
            return await self._recv(timeout=timeout)

        try:
            if timeout is None:
                res = await self.websocket.recv()
            else:
                res = await asyncio.wait_for(self.websocket.recv(), timeout=timeout)
        except websockets.ConnectionClosedError:
            logger.warning(f'ConnectionClosedError recieving data, reconnecting...')
            await self._connect()
            return await _retry()

        res_dict = self._decode(res)

        if "status" in res_dict:
            assert res_dict["status"] == "ok", res_dict["status"]
            return await _retry()

        res_dict_or_None = await self._pingpong(res_dict)
        if res_dict_or_None is not None:
            return res_dict_or_None

        return await _retry()

    async def _pingpong(self, data: dict) -> Optional[dict]:
        if 'ping' in data:
            n = data['ping']
            logger.info(f'Recieved ping {n}, sending pong...')
            await self._send({'pong': n})
            return None

        return data

    async def _connect(self):
        logger.info(f'Connecting...')
        try:
            self.websocket = await websockets.connect(self.uri)
        except ConnectionError:
            logger.info(f'ConnectionError {ConnectionError}, retrying...')
            return await self._connect()

        for market in self.markets:
            logger.info(f'subscribing {market}')
            await self._send({"sub": f"market.{market}.trade.detail", "id": market})

    async def recv_price(self, timeout=None):
        res = await self._recv(timeout=timeout)
        market = res['ch'].replace('market.', '').replace('.trade.detail', '')
        return market, res['tick']['data'][0]['price']


async def main():
    hb = Huobi(markets=['btcusdt', 'ethusdt'])
    await hb._connect()
    while True:
        print(f"{await hb.recv_price()}\r", end='', flush=False)


if __name__ == "__main__":
    asyncio.run(main())
