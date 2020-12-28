import serial
import random


def to_bytes(line1=' ' * 16, line2=' ' * 16):
    assert isinstance(line1, str) and isinstance(
        line2, str
    ), f"{len(line1) =}, {len(line2) =}"
    assert len(line1) == 16 and len(line2) == 16
    return b'\xfeH' + line1.encode() + b'\x00\x00\x00\x00' + line2.encode() + b'\xff'


with serial.Serial('COM3') as ser:
    while True:
        h = to_bytes(f'BTC:{random.randint(0, 9999999) / 100:>12.2f}')
        ser.write(h)
        print('sent', h)
