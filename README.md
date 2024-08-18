# Simple Calculator in Rust

This is a simple command-line calculator written in Rust. It repeatedly prompts the user to enter two numbers and an operator, performs the calculation, and displays the result. The user can continue performing calculations or exit the program.

## Features

- Supports basic arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
- Handles invalid input gracefully.
- Prevents division by zero.
- Allows the user to perform multiple calculations in a single session.

## Usage

1. **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. **Build the project:**
    ```sh
    cargo build
    ```

3. **Run the project:**
    ```sh
    cargo run
    ```

4. **Follow the prompts:**
    - Enter the first number.
    - Enter an operator (`+`, `-`, `*`, `/`).
    - Enter the second number.
    - The result will be displayed.
    - You will be asked if you want to perform another calculation. Enter `yes` to continue or `no` to exit.

## Example

```sh
Enter first number:
5
Enter an operator (+, -, *, /):
+
Enter second number:
3
The result is: 8
Do you want to perform another calculation? (yes/no)
no
