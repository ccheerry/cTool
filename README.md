# cTool

This Rust tool helps you create a structured resource directory with predefined templates for a Cfx.re project.

## Features

- Prompts for resource name and username
- Creates directories: `client`, `server`, and `data`
- Reads template files for `fxmanifest.lua`, `client.lua`, `server.lua`, and `data.lua`
- Replaces placeholders in the templates with user input
- Optionally includes `ox_lib` and a web project in the `fxmanifest.lua`

## Usage

1. **Clone the repository:**

    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. **Prepare the templates:**

    Create the following templates you want, following your needs. Atleast do the fxmanifest one or use the one provided on the repo
    - `fxmanifest.lua`
    - `client.lua`
    - `server.lua`
    - `data.lua`

3. **Run the program:**

    ```sh
    cargo run
    ```
    **Or to build an executable:**
    ```sh
    cargo build --release
    ```

5. **Follow the prompts:**

    - Input the desired resource name.
    - Input your name (to display on `fxmanifest`).
    - Answer the prompts about including `ox_lib` and a web project.

## Example

```sh
Input the desired resource name:
tmh_utils
Input your name (to display on fxmanifest):
Cherry
Creating files for resource named: my_resource
Import ox_lib? (yes/no)
yes
Include web project? (yes/no)
no
Resource created successfully!
