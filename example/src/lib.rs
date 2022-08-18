use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

use chrome_native::{chrome_native_data, chrome_native_task};
use chrome_native::{parse_data, Plugin};

/// Task Data
#[chrome_native_data]
pub struct FileCreation {
    file_name: String,
    content: String,
}

/// Tasks that will be handled from chrome
#[chrome_native_task]
pub enum Tasks {
    HelloTask(String),
    FileCreateTask(FileCreation),
}

#[derive(Plugin)]

pub struct MyPlugin {
    /// Data to be used as a plugin
    #[allow(dead_code)]
    id: String,
}

impl MyPlugin {
    /// A method that creates an instance of self
    fn create_self() -> Self {
        Self {
            id: String::from("<MY_ID>"),
        }
    }
}

impl Plugin for MyPlugin {
    /// Handle raw chrome messages
    fn handle_command(&self, command: String) -> Result<String, Box<dyn Error>> {
        let task = parse_data::<Tasks>(command.as_str())?;
        match task {
            Tasks::HelloTask(str) => Ok(str),
            Tasks::FileCreateTask(creation) => {
                let file = File::create(&creation.file_name)?;
                let mut file = BufWriter::new(file);
                file.write_all(creation.content.as_bytes())?;
                Ok(format!(
                    "Created file: {}, content: {}",
                    &creation.file_name, &creation.content
                ))
            }
        }
    }
}
