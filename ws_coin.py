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

    async def _recv(self) -> dict:
        res = await self.websocket.recv()
        res_dict = self._decode(res)
        res_dict_or_None = await self._pingpong(res_dict)
        if res_dict_or_None is not None:
            return res_dict_or_None

        return await self._recv()

    async def _pingpong(self, data: dict) -> Optional[dict]:
        if 'ping' in data:
            n = data['ping']
            await self._send({'pong': n})
            return None

        return data

    async def _connect(self):
        self.websocket = await websockets.connect(self.uri)
        await self._send({"sub": "market.btcusdt.trade.detail", "id": 'whatever'})
        res = await self._recv()
        assert res["status"] == "ok", res["status"]

    async def recv_price(self):
        res = await self._recv()
        return res['tick']['data'][0]['price']


async def main():
    hb = Huobi()
    await hb._connect()
    while True:
        print(f"{await hb.recv_price()}\r", end='', flush=False)


if __name__ == "__main__":
    asyncio.run(main())
