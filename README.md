# This program was made specifically for the Acer Predator Helios 300 notebook, model ph315-55, which has per-key RGB. It is likely to work on other models from acer that also have per-key RGB.

## 🎛️ Application Commands

| Command              | Description                                      | Example                          |
|----------------------|------------------------------------------------|----------------------------------|
| `color R G B`       | Sets a static color (RGB).                     | `cargo run -- color 255 0 0`            |
| `brightness up`     | Increases brightness.                           | `cargo run -- brightness up`                 |
| `brightness down`   | Decreases brightness.                           | `cargo run -- brightness down`               |
| `effects R G B`     | Activates dynamic effects in the specified color. | `cargo run -- effects 0 0 255`  |
| `speed up`         | Increases the speed of dynamic effects.         | `cargo run -- speed up`                      |
| `speed down`       | Decreases the speed of dynamic effects.         | `cargo run -- speed down`                    |
| `reset`            | Resets the state file to default settings.       | `cargo run -- reset`                         |

📌 **Note:** The `R G B` values must be in the range `0-255`.

