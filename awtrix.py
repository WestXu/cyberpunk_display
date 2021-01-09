import json
from time import sleep

import requests


def push(data: dict, endpoint: str):
    res = requests.post(
        f'http://localhost:7000/api/v3/{endpoint}',
        data=json.dumps(data),
        headers={'Content-Type': 'application/json'},
    )
    return res.text


def notify_price(price: int):
    str_price = str(price)
    assert len(str_price) == 5
    push(
        {
            "force": True,
            "icon": 240,
            "text": str_price,
            'scrollSpeed': 10,
        },
        endpoint='notify',
    )


def draw_exit():
    push(
        {
            "draw": [{"type": "exit"}],
        },
        endpoint='draw',
    )


def draw_price(price: int):
    str_price = str(price)
    assert len(str_price) == 5
    push(
        {
            "draw": [
                {"type": "fill", "color": [50, 50, 50]},
                {
                    "type": "text",
                    "string": str_price,
                    "position": [4, 1],
                    "color": [255, 0, 0],
                },
                {"type": "show"},
            ],
        },
        endpoint='draw',
    )


if __name__ == "__main__":
    for i in range(1, 9):
        draw_price(i * 11111)
        sleep(0.1)

    draw_exit()
