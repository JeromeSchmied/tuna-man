# 🏓 Tuna Man: tournament manager

> NOTE: WIP

**Tuna Man** is a Rust-powered CLI/TUI application that creates tournaments and manages, initially built for table tennis.
Whether you're organizing a casual game night or a competitive event, it helps you manage tournaments with ease. 

## Features

- 💾 **CSV-Based Input**: Easily import players or teams from a `.csv` file.
- 🔓 **Flexibility**: Designed for table tennis tournaments in our school, but it can be used for any tournament.
- 🏆 **Multiple Formats**: Can automatically create brackets for multiple tournament formats.

> **_TODO_**
> - [ ] 🖥️ **TUI interface**: Integration with [ratatui](https://ratatui.rs) for a sleek terminal user interface is in the works.
> - 🔄 **More Tournament formats**: Upcoming support for multiple tournament formats eg.:
>   - [x] double-elemination
>   - [x] single-elimination
>   - [x] Round-robin
>   - [ ] Swiss-system
>   - [ ] any with seeding
> - [ ] library?

## Getting Started

### Prerequisites

- **Rust** installed (<href>https://www.rust-lang.org/tools/install</href>)
- A CSV file with participants in `<player/team>,<class>` format (where `<class>` is optional)

### Installation

Clone the repository:

```bash
git clone https://codeberg.org/jark/tuna-man.git
cd tuna-man
```

Build the project:

```bash
cargo build --release
```

All-in-one easy mode:  
```bash
cargo install --locked --git "https://codeberg.org/jark/tuna-man"
```

### Usage

To create a tournament, simply run the following command, providing the path to your `.csv` file:

```bash
tuna-man <FILE>
```

- example input file with class
  ```csv
  name,class
  Alice,11A
  Bob,9B
  Jennice,0C
  ...
  ```
- example input file without class
  ```csv
  name
  Alice
  Bob
  Jennice
  ...
  ```

### Notable Options:

- `-h`, `--help`: Display help message with usage details.
