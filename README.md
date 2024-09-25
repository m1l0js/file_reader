# File_reader
### Project Description

This Rust project is a simple command-line application that reads the contents of a file line by line and prints them to the console. It takes user input to determine the file to read, handles potential file-related errors (such as the file not existing), and prints appropriate error messages if any issues occur. The program uses the standard library’s `File` and `BufReader` to efficiently read large files in a buffered manner.

### Key Features:
- **File Input**: The program prompts the user to enter the name of a file to be read.
- **Error Handling**: If the file does not exist or an error occurs during reading, the program gracefully handles the error and provides feedback to the user.
- **Line-by-Line Reading**: The file is read line by line using buffered reading, which is efficient and suitable for large files.
- **Rust's Ownership Model**: The program demonstrates Rust's memory safety by handling file I/O operations safely without memory leaks or unsafe access.

---

# Rust File Reader

This Rust project is a command-line tool that reads the contents of a file line by line and displays them in the terminal. It is a simple example of handling file I/O operations in Rust using the standard library.

## Features

- Prompts the user to input the name of the file to read.
- Gracefully handles errors like file not found or permission issues.
- Efficiently reads the file using buffered I/O.
- Prints each line of the file to the console.
  
## How to Use

### Prerequisites

- Ensure that you have [Rust](https://www.rust-lang.org/) installed on your machine.

### Running the Program

1. Clone this repository or copy the source code.
2. Compile the program using `cargo build` or run it directly with `cargo run` from the project directory.
   
   ```bash
   cargo run
   ```

3. When prompted, enter the full path of the file you want to read.
   
   Example:
   
   ```bash
   Enter the name of the file you want to read:
   sample.txt
   ```

4. The program will print the contents of the file line by line.

### Error Handling

- If the file does not exist or cannot be opened, the program will print an error message, such as:

   ```bash
   The file you entered does not exist
   ```

- If there is an error reading a particular line, the program will display a message like:

   ```bash
   Failed to read line: <error message>
   ```

## Example

Here's an example of running the program and reading a sample text file:

```bash
Enter the name of the file you want to read:
example.txt
Line 1: Hello, World!
Line 2: This is a sample file.
Line 3: It contains several lines of text.
```

## Project Structure

- `src/main.rs`: Contains the core logic of the program.
  
## Contributing

Feel free to open issues or submit pull requests for improvements or bug fixes. Contributions are always welcome!

