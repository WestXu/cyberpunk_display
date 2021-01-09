import asyncio
import json

import aiohttp


async def push(ssn: aiohttp.ClientSession, data: dict, endpoint: str):
    async with ssn.post(
        f'http://localhost:7000/api/v3/{endpoint}',
        data=json.dumps(data),
        headers={'Content-Type': 'application/json'},
    ) as res:
        return res.text


async def draw_exit(ssn: aiohttp.ClientSession):
    await push(
        ssn,
        {
            "draw": [{"type": "exit"}],
        },
        endpoint='draw',
    )


async def draw_price(ssn: aiohttp.ClientSession, price: int):
    str_price = str(price)
    assert len(str_price) == 5
    await push(
        ssn,
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

    async def main():
        async with aiohttp.ClientSession() as ssn:
            for i in range(1, 9):
                await draw_price(ssn, i * 11111)
                await asyncio.sleep(0.1)

            await draw_exit(ssn)

    asyncio.run(main())
