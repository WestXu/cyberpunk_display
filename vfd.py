import asyncio
from typing import Dict, Literal

import serial
from loguru import logger

from ws_coin import Huobi


def to_bytes(line1=' ' * 16, line2=' ' * 16):
    assert isinstance(line1, str) and isinstance(line2, str)
    assert len(line1) == 16 and len(line2) == 16, f"{len(line1) =}, {len(line2) =}"
    return b'\xfeH' + line1.encode() + b'\x00\x00\x00\x00' + line2.encode() + b'\xff'


class Coin:
    def __init__(self, market, name, precision: Literal[1, 2, 3, 4] = 2):
        self.market = market
        self.name = name.upper()
        self.precision = precision

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
        str_p = ('{:.%df}' % self.precision).format(self.p)
        return f"{self.name + ':':<6}{self.trend}{str_p:>9}"


class VFD:
    def __init__(self, ser, loop) -> None:
        self.ser = ser

        self._last_sent = to_bytes()
        self._q: asyncio.Queue = asyncio.Queue(maxsize=1)

        self.loop = loop

    async def update(self, msg: bytes):
        if not self._q.empty():
            await self._q.get()

        await self._q.put(msg)

    async def send(self, msg: bytes):
        await self.loop.run_in_executor(None, self.ser.write, msg)
        logger.info(f'Sent to VFD')

    async def send_latest(self, timeout=0.5):
        try:
            msg = await asyncio.wait_for(self._q.get(), timeout)
            self._last_sent = msg
        except asyncio.TimeoutError:
            logger.warning('timeout')
            msg = self._last_sent

        await self.send(msg)


class Driver:
    def __init__(self, vfd: VFD, coins: Dict[str, Coin], loop) -> None:
        self.vfd = vfd

        assert len(coins) >= 2
        self.coins = coins
        self.loop = loop

        self.hb = Huobi(markets=list(coins.keys()))

        self.show_coins = list(coins.keys())[:2]

    async def connect_hb(self):
        await self.hb._connect()

    async def switch_run(self):
        while True:
            for market in list(self.coins.keys())[1:]:
                self.show_coins[1] = market
                await asyncio.sleep(2)

    async def recv_run(self):
        while True:
            market, p = await self.hb.recv_price()
            logger.info(f"{market} {p}")
            self.coins[market].update(p)

            if market in self.show_coins:
                await vfd.update(
                    to_bytes(
                        self.coins[self.show_coins[0]].line,
                        self.coins[self.show_coins[1]].line,
                    )
                )

    async def push_run(self):
        while True:
            await self.vfd.send_latest()

    async def start(self):
        await self.connect_hb()

        if len(self.coins) > 2:
            self.loop.create_task(self.switch_run())

        self.loop.create_task(self.recv_run())

        await self.push_run()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    with serial.Serial('COM6') as ser:
        vfd = VFD(ser, loop)
        coins = {
            'ethusdt': Coin('ethusdt', 'ETH'),
            'ltcusdt': Coin('ltcusdt', 'LTC'),
            'uniusdt': Coin('uniusdt', 'UNI', 4),
            'bagsusdt': Coin('bagsusdt', 'BAGS'),
        }
        driver = Driver(vfd, coins, loop)
        loop.run_until_complete(driver.start())
