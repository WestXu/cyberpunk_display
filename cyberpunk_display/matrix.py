from .cyberpunk_display import PriceQueueRust, WsCoinRust


def main():
    pq = PriceQueueRust()

    print("\n\n\n\n\n\n\n\n")
    for p in WsCoinRust():
        pq.push(p)
        print(f"\x1b[8A{pq.to_plot()}")
