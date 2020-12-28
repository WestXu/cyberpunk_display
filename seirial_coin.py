import asyncio
import serial
from ws_coin import Huobi


def to_bytes(line1=' ' * 16, line2=' ' * 16):
    assert isinstance(line1, str) and isinstance(
        line2, str
    ), f"{len(line1) =}, {len(line2) =}"
    assert len(line1) == 16 and len(line2) == 16
    return b'\xfeH' + line1.encode() + b'\x00\x00\x00\x00' + line2.encode() + b'\xff'


async def main():
    hb = Huobi()
    await hb._connect()

    with serial.Serial('COM3') as ser:
        p = 0
        trend: Literal['+', '-'] = '+'
        while True:
            try:
                new_p = await hb.recv_price(timeout=0.5)
                if new_p > p:
                    trend = '+'
                elif new_p < p:
                    trend = '-'
                p = new_p
            except asyncio.TimeoutError:
                pass
            print(f"{p} {trend}\r", end='', flush=True)
            ser.write(to_bytes(f"      BTC {trend}     ", f"{p:^16.2f}"))


if __name__ == "__main__":
    asyncio.run(main())
