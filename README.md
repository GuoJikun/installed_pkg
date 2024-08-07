# installed_pkg

> Under development, do not use for official environment.

A simple cross-platform crate that lists all the apps installed on a system.

![Crates.io License](https://img.shields.io/crates/l/installed_pkg)
![Crates.io Version](https://img.shields.io/crates/v/installed_pkg)

## Usage

### add `installed_pkg` to your project `Cargo.toml`

```bash
cargo add installed_pkg
```

```rs
use installed_pkg::list as app_list;

fn main() {
    let installed = app_list();
    println!("apps: {:?}", installed.apps);
}
```

## Supported platforms

| Platform | status    |
| -------- | --------- |
| Windows  | Done      |
| Macos    | Done      |
| Linux    | come soon |

> In windows platform, the data whose name and application root directory are empty are filtered.