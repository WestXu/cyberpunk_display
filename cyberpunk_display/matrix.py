import asyncio

from .cyberpunk_display import PriceQueueRust


def main():
    import random

    async def main():
        p = 0
        pq = PriceQueueRust()
        while True:
            p += random.randint(-10, 10)
            pq.push(p)
            print(pq.to_plot())
            await asyncio.sleep(0.1)

    asyncio.run(main())
