import asyncio
from time import sleep
from typing import Literal, Union

import serial
from loguru import logger

from vfd import to_bytes
from ws_coin import Huobi


def format_num(num: float) -> float:
    num = float(num)
    assert num > 0

    big, small = str(num).split('.')
    assert len(big) == 5

    return round(num, 1)


def to_bytes(num: float) -> bytes:
    num_str = f"{num:0>6}"

    dot_list = list('BBBBBB')
    if (dot_position := num_str.find('.')) != -1:
        dot_list[dot_position] = 'L'

    return b'TIMD' + num_str.replace('.', '').encode() + ''.join(dot_list).encode()


class Nixie:
    def __init__(self, com_port: int, loop) -> None:
        self.com_port = com_port
        self.loop = loop

        self._q: asyncio.Queue = asyncio.Queue(maxsize=1)
        self._last_sent = 0

    def __enter__(self):
        self.ser = serial.Serial(f'COM{self.com_port}')
        return self

    def __exit__(self, exc_type, exc, tb):
        self.ser.write(f'TIMDBBBBBBBBBBBB'.encode())  # 关闭辉光管的所有灯丝
        sleep(1)
        self.ser.close()

    def set_brightness(self, brightness: int = 9):
        assert 0 <= brightness <= 9
        self.ser.write(f'TIMB{brightness}'.encode())

    async def update(self, p: float):
        if not self._q.empty():
            await self._q.get()

        await self._q.put(format_num(p))

    async def send(self, p: float):
        await self.loop.run_in_executor(None, self.ser.write, to_bytes(p))
        logger.info(f'Sent to Nixie')

    async def send_latest(self):
        p = await self._q.get()
        if p != self._last_sent:
            await self.send(p)
            self._last_sent = p


async def data(nixie: Nixie):
    hb = Huobi(markets=['btcusdt'])
    await hb._connect()

    while True:
        market, p = await hb.recv_price()
        logger.info(f"{market} {p}")
        await nixie.update(p)


async def push(nixie: Nixie):
    while True:
        await nixie.send_latest()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    with Nixie(5, loop) as nixie:
        nixie.set_brightness(9)

        loop.create_task(data(nixie))
        loop.run_until_complete(push(nixie))
