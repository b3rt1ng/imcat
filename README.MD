# imcat

Want to customize your terminal utilities to a comically high degree ? Well, **imcat** is a lightweight Rust utility for displaying images in the terminal. It leverages the power of Rust, the colored and image crates, allowing users to view images in their terminal using character-based representations. With imcat, you can resize and display images, making it a handy tool for quick image previews and exploration.

## Features
- Resize images to fit your terminal width.
- Display images using colored characters for a pixel-like appearance.
- Written in Rust for performance and efficiency.

## Usage
1. Clone the repository.
2. Build the project using `cargo build --release`.
3. Move the generated executable to a directory in your PATH.
   ``` bash
   sudo cp target/release/imcat /usr/bin/
   sudo cp target/release/imcat /bin
   ```
5. Run `imcat path/to/your/image.jpg` to view your images in the terminal.

Explore and enjoy the simplicity of image viewing directly in your terminal with imcat !
