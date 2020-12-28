import asyncio
import gzip
import json
from typing import Optional

import websockets


class Huobi:
    uri = "wss://api.hadax.com/ws"

    @staticmethod
    def _decode(msg: bytes) -> dict:
        return json.loads(gzip.decompress(msg))

    async def _send(self, data: dict):
        await self.websocket.send(json.dumps(data))

    async def _recv(self, timeout=None) -> dict:
        if timeout is None:
            res = await self.websocket.recv()
        else:
            res = await asyncio.wait_for(self.websocket.recv(), timeout=timeout)
        res_dict = self._decode(res)

        if "status" in res_dict:
            assert res_dict["status"] == "ok", res_dict["status"]
            return await self._recv(timeout=timeout)

        res_dict_or_None = await self._pingpong(res_dict)
        if res_dict_or_None is not None:
            return res_dict_or_None

        return await self._recv(timeout=timeout)

    async def _pingpong(self, data: dict) -> Optional[dict]:
        if 'ping' in data:
            n = data['ping']
            await self._send({'pong': n})
            return None

        return data

    async def _connect(self):
        self.websocket = await websockets.connect(self.uri)

    async def _sub(self, market="btcusdt"):
        await self._send({"sub": f"market.{market}.trade.detail", "id": market})

    async def recv_price(self, timeout=None):
        res = await self._recv(timeout=timeout)
        market = res['ch'].replace('market.', '').replace('.trade.detail', '')
        return market, res['tick']['data'][0]['price']


async def main():
    hb = Huobi()
    await hb._connect()
    await hb._sub('btcusdt')
    await hb._sub('ethusdt')
    while True:
        print(f"{await hb.recv_price()}\r", end='', flush=False)


if __name__ == "__main__":
    asyncio.run(main())
