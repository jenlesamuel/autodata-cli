# AUTODATA CLI
AUTODATA CLI is a command-line interface (CLI) tool built in Rust for fetching and filtering details of car manufacturers. It consumes an endpoint that returns a list of car manufacturers and allows users to filter the list by providing a search term as a command line argument.

## Features
- **Fetch Manufacturer Details**: Retrieves a comprehensive list of car manufacturers from an endpoint.
- **Filtering**: Allows filtering of the manufacturer list by a provided search term.
- **Performance**: Written in Rust, ensuring high performance and reliability.

## Installation
#### Clone the repository
```shell
git clone https://github.com/jenlesamuel/autodata-cli
cd autodata-cli
```

#### Build the project
Make sure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).
```shell
cargo build --release
```

#### Run the executable
```shell
./target/release/autodata-cli
```

## Usage
#### Basic Usage
To fetch the complete list of car manufacturers:
```shell
./autodata-cli
```

#### Filtering
To filter the list of car manufacturers by a search term (e.g., "Toyota"):
```shell
./autodata-cli BMW
```
