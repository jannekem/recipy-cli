# recipy-cli

This is a command line utility for importing recipes from the web into your Recipy site.

Recipy is a recipe management theme for Hugo. Recipes are stored as Markdown files that contain front matter with metadata about the recipe. This utility allows you to import recipes from websites that support the [Recipe Schema](https://schema.org/Recipe) into your Recipy site.

## Installation

You can download the latest binary for your platform from the [releases page](https://github.com/jannekem/recipy-cli/releases). Extract the archive and move the binary to a location in your PATH.

Alternatively, you can build the binary from source by cloning this repository, installing Rust and running `cargo build --release` in the repository root. The binary will be located in `target/release` and can be moved to a location in your PATH.

## Usage

Run `recipy-cli --help` to see the available commands and options. The utility requires a Recipy site to be present in the current directory. Attempting to run the utility in a directory without the required directory structure will result in an error.

### Importing a recipe

To import a recipe, you can simply run `recipy-cli <URL>`. The utility will attempt to download the recipe from the given URL and import it into your Recipy site. Check the help for more options to customize the import.
