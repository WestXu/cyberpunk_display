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
    hb = Huobi()
    await hb._connect()
    await hb._sub('btcusdt')
    await hb._sub('ethusdt')

    btc = Coin('btcusdt', 'BTC')
    eth = Coin('ethusdt', 'ETH')

    with serial.Serial('COM3') as ser:
        while True:
            try:
                market, p = await hb.recv_price(timeout=0.5)
            except asyncio.TimeoutError:
                continue

            logger.info(f"{market} {p}")

            assert market in {'btcusdt', 'ethusdt'}, f"unknown market {market}"
            if market == 'btcusdt':
                btc.update(p)
            else:
                eth.update(p)

            ser.write(to_bytes(btc.line, eth.line))


if __name__ == "__main__":
    asyncio.run(main())
