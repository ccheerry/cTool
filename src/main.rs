use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    println!("Input the desired resource name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed reading input");
    let name = name.trim();

    println!("Input your name (to display on fxmanifest):");

    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed reading input");
    let username = username.trim();

    println!("Creating files for resource named: {}", name);

    fs::create_dir_all(format!("{}/client", name)).expect("Failed creating client directory");
    fs::create_dir_all(format!("{}/server", name)).expect("Failed creating server directory");
    fs::create_dir_all(format!("{}/data", name)).expect("Failed creating data directory");

    fn read_template(file_path: &Path) -> String {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .expect(&format!("Failed to read template file: {:?}", file_path));
                content
            }
            Err(_e) => {
                String::new() // Creates the file anyways but without any template
            }
        }
    }

    // Prompt handling
    fn prompt_yes_no(question: &str) -> bool {
        println!("{}", question);
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed reading input");
        matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
    }

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let fxmanifest_template = read_template(&current_dir.join("templates").join("fxmanifest.lua"));
    let client_template = read_template(&current_dir.join("templates").join("client.lua"));
    let server_template = read_template(&current_dir.join("templates").join("server.lua"));
    let data_template = read_template(&current_dir.join("templates").join("data.lua"));

    // Replace placeholders in the templates
    let mut fxmanifest_content = fxmanifest_template
        .replace("{filename}", name)
        .replace("{yourname}", username);

    if prompt_yes_no("Import ox_lib? (yes/no)") {
        fxmanifest_content.push_str("\nshared_script {\n    '@ox_lib/init.lua'\n}\n");
    }

    if prompt_yes_no("Include web project? (yes/no)") {
        fxmanifest_content.push_str("\nui_page {\n    'web/dist/index.html'\n}\n\nfiles {\n    'data/*.lua',\n    'web/dist/**/*'\n}\n",
        );
    }

    // Write the templates text into the files
    let fxmanifest_path = format!("{}/fxmanifest.lua", name);
    let mut fxmanifest = File::create(&fxmanifest_path).expect("Failed creating fxmanifest.lua");
    fxmanifest
        .write_all(fxmanifest_content.as_bytes())
        .expect("Failed writing to fxmanifest.lua");

    let client_path = format!("{}/client/client.lua", name);
    let mut client = File::create(&client_path).expect("Failed creating client.lua");
    client
        .write_all(client_template.as_bytes())
        .expect("Failed writing to client.lua");

    let server_path = format!("{}/server/server.lua", name);
    let mut server = File::create(&server_path).expect("Failed creating server.lua");
    server
        .write_all(server_template.as_bytes())
        .expect("Failed writing to server.lua");

    let data_path = format!("{}/data/data.lua", name);
    let mut data = File::create(&data_path).expect("Failed creating data.lua");
    data.write_all(data_template.as_bytes())
        .expect("Failed writing to data.lua");

    println!("Resource created successfully!");
}
