# Rust ASCII Art Generator

This Rust project is a simple yet fun application that generates ASCII art from images. It uses the OpenAI API to generate images based on text prompts and then converts these images into ASCII art.

## Features

- **Image Generation**: Utilizes OpenAI's image generation capabilities to create images based on user-provided prompts.
- **ASCII Art Conversion**: Converts the generated images into ASCII art.

## Prerequisites

- Rust programming environment.
- `openai_api_rs` crate for OpenAI API integration.
- `image` crate for image processing.
- `reqwest` crate for HTTP requests.

## Installation

1. Clone the repository:

    ```
    git clone https://github.com/leoforney/dalle-ascii
    ```

2. Navigate to the project directory:

    ```
    cd dalle-ascii
    ```

3. Build the project:

    ```
    cargo build --release
    ```

## Usage

1. Set your OpenAI API key as an environment variable:

    ```
    export OPENAI_API_KEY=[Your OpenAI API Key]
    ```

2. Run the program with a prompt:

    ```
    ./target/release/[program name] "Your image prompt"
    ```

   Ensure to enclose the prompt in double quotes if it contains spaces.

3. The program will output ASCII art based on the given prompt.

## Contributing

Feel free to fork the repository and submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- OpenAI for their image generation API.
- Creators of the `openai_api_rs`, `image`, and `reqwest` crates.
