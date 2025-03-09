# PNGSpy

PNGSpy is a command-line tool designed to manipulate PNG files by encoding, decoding, and removing custom data chunks. It supports operations such as encoding messages into PNG files, decoding messages from PNG files, and removing specific chunks from PNG files. The tool is cross-platform and can be used on Windows, macOS, and Linux.

## Features

- **Encode Messages**: Embed secret messages into PNG files using custom chunk types.
- **Decode Messages**: Extract and display messages from specified chunks in PNG files.
- **Remove Chunks**: Remove specific chunks from PNG files.
- **Print PNG Structure**: Display the structure of a PNG file, including its chunks.
- **Download and Encode**: Download a PNG from a URL and encode a message into it.

## Installation

### Prerequisites

- Rust and Cargo must be installed. You can download them from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd pngspy
   ```
2. Build and install the project:
   ```bash
   cargo install --path .
   ```

## Usage

After building and installing the project, you can use the `pngspy` command to perform various operations. Below are some examples:

### Encode a Message
Embed a secret message into a PNG file. The command takes the PNG file path, a 4-character chunk type, and the message. Optionally, specify an output path.
```bash
pngspy encode <file_path> <chunk_type> <message> [-o <output>]
```
Example:
```bash
pngspy encode ./samples/input.png ruSt "secret message" -o ./samples/output.png
```
or
```bash
pngspy encode ./samples/input.png ruSt "secret message"
```

### Decode a Message
Extract a secret message from a PNG file by specifying the PNG file path and the chunk type.
```bash
pngspy decode <file_path> <chunk_type>
```
Example:
```bash
pngspy decode ./samples/input.png ruSt
```

### Remove a Chunk
Remove a specific chunk from a PNG file by providing the PNG file path and the chunk type.
```bash
pngspy remove <file_path> <chunk_type>
```
Example:
```bash
pngspy remove ./samples/input.png ruSt
```

### Print PNG Structure
Display the structure of a PNG file (including its header and chunks).
```bash
pngspy print <file_path>
```
Example:
```bash
pngspy print ./samples/input.png
```

### Encode a Message from a URL
Download a PNG from a URL and embed a secret message into it. This command requires the image URL, a 4-character chunk type, and the message.
```bash
pngspy encode-url <url> <chunk_type> <message> -o <output>
```
Example:
```bash
pngspy encode-url https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png ruSt "secret message" -o ./samples/output.png
```

## Project Structure

- **src/**: Contains the source code for the project.
  - **args/**: Command-line argument parsing.
  - **commands/**: Command handlers for each operation.
  - **chunks/**: Definitions and operations related to PNG chunks.
  - **network/**: Network operations, such as downloading images.
  - **utils/**: Utility functions, such as CRC calculations.
  - **png/**: Core PNG file operations.
  - **error/**: Error handling.
  - **bin/**: Main entry point for the command-line tool.
- **tests/**: Contains integration and unit tests for the project.
- **Cargo.toml**: Project metadata and dependencies.

## Contributions

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes.
4. Ensure all tests pass by running `cargo test`.
5. Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, please open an issue on the GitHub repository. 