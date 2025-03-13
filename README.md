# clicr - Command Line Interface Reference (Rust)

`clicr` is a command-line tool written in Rust that allows you to quickly search and access a database of commonly used command-line commands. It stores command information in a JSON file (commands.json), making it easy to find and manage commands efficiently.

## Features

* **Fast Command Search:** Quickly search for commands by name, description, or language.
* **Clear Command Display:** Displays command name, description, usage, and language.
* **Extensible Database:** Easily add, modify, or remove commands in the `commands.json` file.
* **Cross-Platform (Rust):** Compiles and runs on various platforms supported by Rust.
* **Performance:** written in rust, it has fast execution times.

## Getting Started

### Prerequisites
Ensure you have the Rust toolchain installed (including cargo). You can install it via:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Or visit rustup.rs for installation instructions.

### Installation

1.  **Clone the repository (or download the files):**

    ```bash
    git clone https://github.com/Freedomwithin/clicr
    cd clicr
    ```

2.  **Build the `clicr` executable using Cargo:**

    ```bash
    cargo build --release
    ```

    This will create an executable located at `target/release/clicr`.

3.  **(Optional)Create an executable script (run_clicr.command):**

    Create a file named `run_clicr.command` in the root directory of your `clicr` project. Add the following line to the file:

    ```bash
    #!/bin/bash
    cd "/clicr"
    cargo run
    ```

    Then, make the script executable:

    ```bash
    chmod +x run_clicr.command
    ```

4.  **Run the `clicr` script:**

    ```bash
    ./run_clicr.command
    ```
or copy the file to desktop/desired location

## Usage

When you run run_clicr.command, you will be prompted to enter a search term to look up a command.

### Modifying Commands

- Add a new command – Append a new JSON object in commands.json.
- Edit an existing command – Modify the relevant entry in commands.json.
- Remove a command – Delete the corresponding JSON object from commands.json.

## Contributing

We welcome contributions! Follow these steps:

Fork the repository.
Create a new branch for your feature or bug fix.
Make your changes and commit them.
Submit a pull request to merge your changes.

- Command-line arguments – Allow users to specify search terms as arguments.
-  Interactive mode – Implement command history and tab completion.
-  Fuzzy search – Support partial matches and handle typos.
-  More command details – Include links to documentation or man pages.
-  Platform-specific commands – Add support for OS-specific commands.
-  Colorized output – Improve readability with syntax highlighting.
-  Clipboard support – Allow users to copy command usage easily.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).


## Contact

## Contact

[Jonathon's Profile](https://freedomwithin.github.io/Jonathon_Porfolio/contact.html)  
[GitHub](https://github.com/Freedomwithin/)

