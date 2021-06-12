import asyncio

from .cyberpunk_display import PriceQueueRust


def main():
    import random

    async def main():
        p = 50000
        pq = PriceQueueRust()

        printed = False
        while True:
            p += random.randint(-1000, 1000) / 100
            pq.push(p)
            print('\x1b[8A' * printed + pq.to_plot())
            printed = True
            await asyncio.sleep(0.1)

    asyncio.run(main())
