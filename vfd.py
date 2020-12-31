import asyncio
from typing import Literal

import serial
from loguru import logger

from ws_coin import Huobi


def to_bytes(line1=' ' * 16, line2=' ' * 16):
    assert isinstance(line1, str) and isinstance(line2, str)
    assert len(line1) == 16 and len(line2) == 16, f"{len(line1) =}, {len(line2) =}"
    return b'\xfeH' + line1.encode() + b'\x00\x00\x00\x00' + line2.encode() + b'\xff'


class Coin:
    def __init__(self, market, name):
        self.market = market
        self.name = name.upper()

        self.p = 0
        self.trend: Literal['+', '-'] = '+'

    def update(self, new_p):
        if new_p > self.p:
            self.trend = '+'
        elif new_p < self.p:
            self.trend = '-'
        self.p = new_p

    @property
    def line(self):
        return f"{self.name}: {self.trend}{self.p:>10.2f}"


class VFD:
    def __init__(self, ser) -> None:
        self.ser = ser

        self._last_sent = to_bytes()
        self._q = asyncio.Queue(maxsize=1)

    async def update(self, msg: bytes):
        if not self._q.empty():
            await self._q.get()

        await self._q.put(msg)

    def send(self, msg: bytes):
        self.ser.write(msg)
        logger.info(f'Sent to VFD')

    async def send_latest(self, timeout=0.5):
        try:
            msg = await asyncio.wait_for(self._q.get(), timeout)
            self._last_sent = msg
        except asyncio.TimeoutError:
            msg = self._last_sent

        self.send(msg)


async def data(vfd: VFD):
    coins = {
        'btcusdt': Coin('btcusdt', 'BTC'),
        'ethusdt': Coin('ethusdt', 'ETH'),
    }

    hb = Huobi(markets=list(coins.keys()))
    await hb._connect()

    while True:
        market, p = await hb.recv_price()
        logger.info(f"{market} {p}")
        coins[market].update(p)
        await vfd.update(
            to_bytes(
                coins['btcusdt'].line,
                coins['ethusdt'].line,
            )
        )


async def push(vfd: VFD):
    while True:
        await vfd.send_latest()


if __name__ == "__main__":
    with serial.Serial('COM4') as ser:
        vfd = VFD(ser)

        loop = asyncio.get_event_loop()

        loop.create_task(data(vfd))
        loop.run_until_complete(push(vfd))
