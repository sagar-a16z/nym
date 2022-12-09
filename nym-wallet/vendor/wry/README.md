<img src=".github/splash.png" alt="WRY Webview Rendering library" />

[![](https://img.shields.io/crates/v/wry?style=flat-square)](https://crates.io/crates/wry) [![](https://img.shields.io/docsrs/wry?style=flat-square)](https://docs.rs/wry/) ![](https://img.shields.io/crates/l/wry?style=flat-square)

Cross-platform WebView rendering library in Rust that supports all major desktop platforms like Windows, macOS, and Linux.

<div align="center">
  <a href="https://gfycat.com/needywetelk">
    <img src="https://thumbs.gfycat.com/NeedyWetElk-size_restricted.gif">
  </a>
</div>

## Overview

Wry connects the web engine on each platform and provides easy to use and unified interface to render WebView. It also re-exports [tao] as a module for event loop and window creation.

[tao]: https://crates.io/crates/tao

## Usage

The minimum example to create a Window and browse a website looks like following:

```rust
fn main() -> wry::Result<()> {
  use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    webview::WebViewBuilder,
  };

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)?;
  let _webview = WebViewBuilder::new(window)?
    .with_url("https://tauri.studio")?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}
```

There are also more samples under `examples`, you can enter commands like following to try them:

```
cargo run --example multi_window
```

For more information, please read the documentation below.

## [Documentation](https://docs.rs/wry)

## Platform-specific notes

All platforms uses [tao](https://github.com/tauri-apps/tao) to build the window, and wry re-export it as application module. Here are the underlying web engine each platform uses, and some dependencies you might need to install.

### Linux

Tao uses [gtk-rs](https://gtk-rs.org/) and its related libraries for window creation and wry also needs [WebKitGTK](https://webkitgtk.org/) for WebView. So please make sure following packages are installed:

#### Arch Linux / Manjaro:

```bash
sudo pacman -S webkit2gtk libappindicator-gtk3
```

#### Debian / Ubuntu:

```bash
sudo apt install libwebkit2gtk-4.0-dev libappindicator3-dev
```

#### Fedora

```bash
sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel
```

### macOS

WebKit is native on macOS so everything should be fine.

If you are cross-compiling for macOS using [osxcross](https://github.com/tpoechtrager/osxcross) and encounter a runtime panic like `Class with name WKWebViewConfiguration could not be found` it's possible that `WebKit.framework` has not been linked correctly, to fix this set the `RUSTFLAGS` environment variable:

```
RUSTFLAGS="-l framework=WebKit" cargo build --target=x86_64-apple-darwin --release
```

### Windows

WebView2 provided by Microsoft Edge Chromium is used. So wry supports Windows 7, 8, and 10.

## License
Apache-2.0/MIT
