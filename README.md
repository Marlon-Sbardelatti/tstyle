# TStyle - Tmux Statusline Colorscheme Picker/Creator

TStyle is a command-line tool written in Rust that simplifies the process of managing tmux statusline colorschemes. With TStyle, you can easily pick existing colorschemes, create new ones, and manage your tmux statusline appearance effortlessly.

## Features

- **Get Colorscheme**: Retrieve a tmux statusline colorscheme by its name.
- **Create Colorscheme**: Create a new tmux statusline colorscheme with a specified name and hexadecimal color values.
- **List Colorschemes**: List all available tmux statusline colorschemes.
- **Help Command**: Display help information about available commands and their usage.

## Installation

Currently, TStyle is not available via a package manager. You can build it from source by cloning this repository and compiling it using Cargo, the Rust package manager.

```bash
git clone https://github.com/your-username/tstyle.git
cd tstyle
cargo build --release
```
## Usage

### Get a tmux statusline colorscheme by its name
- **tstyle -g <colorscheme_name>**

### Create a new tmux statusline colorscheme
**Make sure 'colorscheme_hex' is enclosed in single quotes**
- **tstyle -c <colorscheme_name> '<colorscheme_hex>'**

### List all tmux statusline colorschemes available
- **tstyle -l**

### Display help information
- **tstyle -h**

## Example 

### Get the "dracula" tmux statusline colorscheme
```bash
tstyle -g dracula
```
### Create a new colorscheme named "mytheme" with hex color "#abcdef"
```bash
tstyle -c mytheme '#abcdef'
```
### List all available tmux statusline colorschemes
```bash
tstyle -l
```
## Contributing
Contributions to TStyle are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.
## License
This project is licensed under the MIT License - see the LICENSE file for details.
