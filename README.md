# TStyle - Tmux Statusline Colorscheme Picker/Creator

TStyle is a command-line tool written in Rust that simplifies the process of managing tmux statusline colorschemes. With TStyle, you can easily pick existing colorschemes, create new ones, and manage your tmux statusline appearance effortlessly.


https://github.com/Marlon-Sbardelatti/tstyle/assets/117592329/481131a8-757d-4800-ba26-16598c7b2e79


## Features

- **Get Colorscheme**: Retrieve a tmux statusline colorscheme by its name.
- **Create Colorscheme**: Create a new tmux statusline colorscheme with a specified name and hexadecimal color values.
- **List Colorschemes**: List all available tmux statusline colorschemes.
- **Help Command**: Display help information about available commands and their usage.

## Installation

Currently, TStyle is not available via a package manager. You can build it from source by cloning this repository and compiling it using Cargo, the Rust package manager.

```bash
git clone git@github.com:Marlon-Sbardelatti/tstyle.git
cd tstyle
cargo build --release
```
## Setting Up Default Theme
Make sure you have [Catppuccin](https://github.com/catppuccin/tmux) mocha flavour tmux theme set as your theme in your tmux configuration. This CLI tool relies on having one colorscheme set as the default.
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
## Sample Themes
If you've already run the TStyle program, you should have a directory called tstyle in your ~/.config directory. Inside this directory, there should be a file named themes.txt. You can paste the following lines into the themes.txt file to add some sample themes:

```plaintext
ayu;#0a0e14
catppuccin;#1e1e2f
tokyodark;#11121d
dracula;#282a36
```
If you haven't run the TStyle program yet, you can follow these steps to set up the directory and file manually:

Create a directory named tstyle in your ~/.config directory.

```bash
mkdir -p ~/.config/tstyle
```
Inside the tstyle directory, create a file named themes.txt.

Open the themes.txt file and paste the sample themes lines provided above.

These sample themes will serve as examples for managing tmux statusline colorschemes using TStyle.
## Contributing
Contributions to TStyle are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.
## License
This project is licensed under the MIT License - see the [LICENSE](https://github.com/Marlon-Sbardelatti/tstyle/blob/master/LICENSE) file for details.
