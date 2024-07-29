# installed_pkg

A simple cross-platform crate that lists all the apps installed on a system.

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
