# Development

## Cargo.toml
As I mentioned before, the plugin should be a dynamic library. You'd need a `[lib]` section which states that the current project is a `cdylib`. An example would be something like,
```toml
[package]
name = "myplugin"
version = "1.0.0"
edition = "2021"

[lib]
name = "myplugin"
crate-type = ["cdylib"]

[dependencies]
chrome-native = { version = "1.0.0", features = ["macros"] }
serde = { version = "1.0.143", features = ["derive"] }
```

## lib.rs
This would be the main plugin file. 

### Plugin
It needs to derive `Plugin` and have a `create_self` function that returns an instance of itself.
```rust
#[derive(Plugin)]
struct MyPlugin {  }

impl MyPlugin {
    fn create_self() -> Self {
        Self {  }
    }
}
```

### chrome-native-data
A `chrome-native-data` is a struct type that can be deserialized from the data given from the browser. It automatically derives serde's `Serialize` and `Deserialize`.
```rust
#[chrome_native_data]
pub struct User {
    name: String,
    nationality: String,
    is_human: bool,
}
```

### Tasks
`Tasks` are like commands. Whenever you send a message from the browser, every plugin will see the message, and handle it if the plugin is the right one to do so. Using `Tasks` is a way that `chrome-native` plugins determine if the plugin is the right one to handle the given data. 

```rust
#[chrome_native_task]
pub enum Tasks {
    SayHelloTask(String),  // The SayHelloTask will receive a string as a parameter
    GetUserInfoTask(User),  // The GetUserInfoTask will receive a User struct as a parameter ... must be a chrome-native-data
}
```

You need to implement the `Plugin` trait to handle commands from the browser. You can use the `parse_data` function to parse the Task from the given raw command. simply returning a `Result<String>` will send a reply to the browser.
```rust
impl Plugin for MyPlugin {
    /// Handle raw chrome messages
    fn handle_command(&self, command: String) -> Result<String, Box<dyn Error>> {
        let task = parse_data::<Tasks>(command.as_str())?;
        match task {
            Tasks::SayHelloTask(str) => Ok(str),    // This will echo the same string to the browser
            Tasks::GetUserInfoTask(user) => {
                let user_info = database::fetch_user(user);
                Ok(format!(
                    "Age: {}, Gender: {}, Address: {}", &user_info.age, &user_info.gender, &user_info.address
                ))  // This will send the formatted string to the browser
            }
        }
    }
}
```

You can do anything with your plugin. If you are facing problems, feel free to leave an issue on GitHub. ^^ 