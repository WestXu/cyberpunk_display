import fire


def matrix(*args, **kwargs):
    from .matrix import main

    main(*args, **kwargs)


async def awtrix(*args, **kwargs):
    from .awtrix import main

    await main(*args, **kwargs)


async def vfd(*args, **kwargs):
    from .vfd import main

    await main(*args, **kwargs)


async def nixie(*args, **kwargs):
    from .nixie import main

    await main(*args, **kwargs)


def main():
    fire.Fire(
        {
            "awtrix": awtrix,
            "matrix": matrix,
            "vfd": vfd,
            "nixie": nixie,
        }
    )


if __name__ == "__main__":
    main()
