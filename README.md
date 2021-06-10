# Cyberpink Display

Show Bitcoin(crypto) prices using [Nixie tube](https://en.wikipedia.org/wiki/Nixie_tube), [Awtrix](https://github.com/awtrix) or [VFD](https://en.wikipedia.org/wiki/Vacuum_fluorescent_display) display technologies, as desktop decors.

This repo is a mixed rust/python project, utilizing [maturin](https://github.com/PyO3/maturin).

## How to use

1. Make sure `python` and `cargo` accessible from command line. 
2. Install [maturin](https://github.com/PyO3/maturin). Then `maturin develop` at `.`
3. `pip install -e .` at `.`
4. `cyberpunk_display awtrix` anywhere.

## Demo
### nixie.py

![Nixie Tube](nixie.gif)

### awtrix.py

![Awtrix](awtrix.gif)

### vfd.py

![VFD](vfd.gif)
