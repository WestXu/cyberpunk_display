from .cyberpunk_display import BtcMatrixRust


def main():
    print("\n\n\n\n\n\n\n\n")
    for plot, _ in BtcMatrixRust():
        print(f"\x1b[8A{plot}")
