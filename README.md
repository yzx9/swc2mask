# SWC2MSAK

Render neuron to mask 3d image stack.

## Usages

Render a binary neuron image stack.

```bash
swc2mask --output=/path/to/tif /path/to/your/swc
```

Render a neuron image stack, with 8-levels of brightness.

```bash
swc2mask --msaa=8 --output=/path/to/tif /path/to/your/swc
```

Renders part of a neuron image stack.

```bash
swc2mask --range=$min_x,$min_y,$min_z,$max_x,$max_y,$max_z --output=/path/to/tif /path/to/your/swc
```

Render a stack of neuron images, aligned with other images (only support `.v3dpbd` now).

```bash
swc2mask --align=/path/to/imgs --output=/path/to/tif /path/to/your/swc
```

Render a neuron image stack, the brightness decays according to the length of path to node 100, and the decay is 0 when the length is 200.

```bash
swc2mask --mode=path_decay --decay=200 --node=100 --output=/path/to/tif /path/to/your/swc
```

## Questions & Issues

If you have any questions, please open a issue on GitHub

## Contribution

PR & Issue Welcome!

## Development

```BASH
# dev
cargo run . --release -- --output=/path/to/tif /path/to/your/swc

# build
cargo build --release
```

## License

[MIT](https://opensource.org/license/mit/)

Copyright (c) 2023-present, Zexin Yuan
