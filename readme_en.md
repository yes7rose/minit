# Usage

## Installation  

   1. Clone this project to your local machine  
   2. Enter the project directory  
   3. Use `cargo install --path .` to install the default location to ~/.cargo/bin/

## Usage

  To ensure that minit can be used in any directory, you need to add the ~/.cargo/bin/ directory to the environment variables.
  Use `-c` to specify the configuration file and `-r` to specify the directory for managing definition files, in the form of:

  `minit -c <knitter/configs.toml> -r <manage_defines_dir>`
  
  The output of running will initialize the mongodb database according to the settings.
  
  You can also use -f to specify a single management definition file, -f and -r cannot be used simultaneously.