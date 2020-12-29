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


async def main():
    coins = {
        'btcusdt': Coin('btcusdt', 'BTC'),
        'ethusdt': Coin('ethusdt', 'ETH'),
    }

    hb = Huobi(markets=list(coins.keys()))
    await hb._connect()

    with serial.Serial('COM4') as ser:
        while True:
            try:
                market, p = await hb.recv_price(timeout=0.5)
                logger.info(f"{market} {p}")
            except asyncio.TimeoutError:
                logger.warning('timeout')
            else:
                coins[market].update(p)
            finally:
                ser.write(
                    to_bytes(
                        coins['btcusdt'].line,
                        coins['ethusdt'].line,
                    )
                )


if __name__ == "__main__":
    asyncio.run(main())
