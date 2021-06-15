import fire


async def matrix():
    from .matrix import main

    await main()


async def awtrix():
    from .awtrix import main

    await main()


async def vfd():
    from .vfd import main

    await main()


async def nixie():
    from .nixie import main

    await main()


def main():
    fire.Fire(
        {
            "awtrix": awtrix,
            "matrix": matrix,
            "vfd": vfd,
            "nixie": nixie,
        }
    )
