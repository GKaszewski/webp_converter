# WEBP converter
Simple CLI tool to convert image or images to WEBP format.

## Prerequisites (for building)

- Rust programming language
- Cargo package manager

## Installation

Clone the repository and navigate into the project folder:

```bash
git clone https://github.com/yourusername/webp_converter.git
cd webp_converter
```

Build the project using Cargo:

```bash
cargo build --release
```

## Usage

### Single Image Conversion

To convert a single image, run:

```bash
./target/release/webp_converter input.jpg output.webp
```

You can also specify quality:

```bash
./target/release/webp_converter input.jpg output.webp --quality=50.0
```

### Batch Conversion

To convert all `.jpg` and `.png` images in a directory, run:

```bash
./target/release/webp_converter input_folder output_folder --batch
```

You can also specify quality for batch conversion:

```bash
./target/release/webp_converter input_folder output_folder --batch --quality=50.0
```

## Options

- `--batch`: Enables batch conversion for all `.jpg` and `.png` images in a directory.
- `--quality=VALUE`: Sets the quality of the output WebP images (0.0 - 100.0).

## License

This project is licensed under the MIT License.