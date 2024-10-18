  Example:
# 🏓 Tournament Manager

**pingpong** is a Rust-powered CLI application that creates double-elimination tournaments, initially built for table tennis. Whether you're organizing a casual game night or a competitive event, it helps you manage tournament brackets with ease. 

Future plans include expanding to other types of tournaments and integrating a TUI using RATAUI for an even smoother experience.

## Features

- 🏆 **Double-Elimination Format**: Automatically creates brackets for double-elimination tournaments.
- 💾 **CSV-Based Input**: Easily import players or teams from a `.csv` file.
- 🔓 **Flexibility**: Designed for table tennis tournaments, but it can be used for any 1v1 match based sports and games.
- 🖥️  **TUI in Development**: Integration with RATAUI for a sleek terminal user interface is in the works.
- 🔄 **Planned Expansions**: Upcoming support for multiple tournament formats and different types of games.

## Getting Started

### Prerequisites

- **Rust** installed (<href>https://www.rust-lang.org/tools/install</href>)
- A CSV file with participants in `<player/team>,<class>` format (where `<class>` is optional)

### Installation

Clone the repository:

```bash
git clone https://codeberg.org/jark/pingpong.git
cd pingpong
```

Build the project:

```bash
cargo build --release
```

### Usage

To create a tournament, simply run the following command, providing the path to your `.csv` file:

```bash
./pingpong <FILE>
```

- example input file
  ```
  Alice,11A
  Bob,09B
  ...
  ```

### Options:

- `-h`, `--help`: Display help message with usage details.

## Roadmap

- **TUI Interface**: Interactive terminal UI using RATAUI (in progress).
- **More Tournament Types**: Round-robin, single-elimination, Swiss-system formats, etc.
- **More Game Types**: Expand support to other sports and games
