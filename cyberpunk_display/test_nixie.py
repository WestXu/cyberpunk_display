import asyncio
from decimal import Decimal

from cyberpunk_display.nixie import Nixie, logger


async def main(loop, interval=0.5):
    with Nixie(5, loop) as nixie:
        await nixie.send(999999)

        logger.info('Testing brightness...')
        for i in range(9):
            logger.info(f'brightness {i}')
            nixie.set_brightness(i)
            await asyncio.sleep(interval)

        await asyncio.sleep(interval)

        logger.info('Testing num...')
        for i in range(10):
            num = 111111 * i
            logger.info(f'num {num}')
            await nixie.send(num)
            await asyncio.sleep(interval)

        logger.info("Testing decimal point...")
        for i in range(6):
            num = Decimal("111111") * Decimal('0.1') ** i
            logger.info(f'num {num}')
            await nixie.send(num)
            await asyncio.sleep(interval)

        logger.info("Testing closing...")

    logger.success('Finished testing.')


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main(loop))
