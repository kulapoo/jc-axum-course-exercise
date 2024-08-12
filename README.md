
# Axum Full Course Example Project

This repository contains the source code from [Jeremy Chone's Axum Full Course on YouTube](https://youtu.be/XZtlD_m59sM?si=iaMVugxb5BvIEGlt), an in-depth guide to building web applications with Axum in Rust. The code closely follows the original tutorial, updated for the latest version of Axum, and serves as a **personal practice guide**.

## Getting Started

To get started with this project, you'll need to have Rust installed. If you haven't set up your Rust environment yet, you can do so by following the [official Rust installation guide](https://www.rust-lang.org/tools/install).

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/axum-course-example.git
    ```

2. **Navigate to the project directory**:
    ```bash
    cd axum-course-example
    ```

3. **Build the project**:
    ```bash
    cargo build
    ```

4. **Run the project**:
    ```bash
    cargo run
    ```

5. Open your browser and navigate to \`http://localhost:3000\` to see the application in action.

### Interactive Development with `cargo watch`

For a more interactive development experience, you can use `cargo watch` with the `tests/quick_dev` file. This allows you to see changes in your code almost immediately:

1. **Install `cargo watch`**:
    ```bash
    cargo install cargo-watch
    ```

2. **Run `cargo watch`** to automatically test your changes:
    ```bash
    cargo watch -q -c -x 'test quick_dev'
    ```

   This command watches for changes in your source files and reruns the `quick_dev` test, giving you instant feedback.

### Additional Commands for Watching Files

If you want to watch specific parts of the code, you can use the following commands:

- **Watch the `main` source code**:

    ```sh
    cargo watch -q -c -w src/ -x run
    ```

    This command will watch for changes in the `src/` directory and automatically run the project whenever a change is detected.

- **Watch the `tests` source code**:

    ```sh
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
    ```

    This command will watch for changes in the `tests/` directory and rerun the `quick_dev` test with output captured.


## Sample
![Demo](./a.gif)

## Project Structure

Here's a brief overview of the project structure:

- **src/**: Contains all the source code for the project.
  - **main.rs**: The entry point of the application.
  - **routes/**: Directory containing the route handlers.
  - **models/**: Directory containing the data models used in the application.
  - **middleware/**: Directory containing any custom middleware.
  - **errors/**: Directory handling custom error types.
- **tests/**: Contains test files.
  - **quick_dev.rs**: A test file for quick and interactive development.

## Contributing

Contributions are welcome! If you have any improvements, feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

A big thank you to [Jeremy Chone](https://www.youtube.com/@JeremyChone) for creating such an amazing course.

