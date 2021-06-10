import fire

from .awtrix import main as awtrix
from .matrix import main as matrix


def main():
    fire.Fire(
        {
            "awtrix": awtrix,
            "matrix": matrix,
        }
    )
