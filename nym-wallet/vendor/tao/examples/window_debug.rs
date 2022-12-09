// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

// This example is used by developers to test various window functions.

use tao::{
  dpi::{LogicalSize, PhysicalSize},
  event::{DeviceEvent, ElementState, Event, KeyEvent, RawKeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{Key, KeyCode},
  window::{Fullscreen, WindowBuilder},
};

#[allow(clippy::single_match)]
#[allow(clippy::collapsible_match)]
fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();

  let window = WindowBuilder::new()
    .with_title("A fantastic window!")
    .with_inner_size(LogicalSize::new(100.0, 100.0))
    .build(&event_loop)
    .unwrap();

  eprintln!("debugging keys:");
  eprintln!("  (E) Enter exclusive fullscreen");
  eprintln!("  (F) Toggle borderless fullscreen");
  eprintln!("  (P) Toggle borderless fullscreen on system's preffered monitor");
  eprintln!("  (M) Toggle minimized");
  eprintln!("  (Q) Quit event loop");
  eprintln!("  (V) Toggle visibility");
  eprintln!("  (X) Toggle maximized");

  let mut minimized = false;
  let mut visible = true;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      // This used to use the virtual key, but the new API
      // only provides the `physical_key` (`Code`).
      Event::DeviceEvent {
        event:
          DeviceEvent::Key(RawKeyEvent {
            physical_key,
            state: ElementState::Released,
            ..
          }),
        ..
      } => match physical_key {
        KeyCode::KeyM => {
          if minimized {
            minimized = !minimized;
            window.set_minimized(minimized);
          }
        }
        KeyCode::KeyV => {
          if !visible {
            visible = !visible;
            window.set_visible(visible);
          }
        }
        _ => (),
      },
      Event::WindowEvent {
        event:
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                logical_key: Key::Character(key_str),
                state: ElementState::Released,
                ..
              },
            ..
          },
        ..
      } => match key_str {
        // WARNING: Consider using `key_without_modifers()` if available on your platform.
        // See the `key_binding` example
        "e" => {
          fn area(size: PhysicalSize<u32>) -> u32 {
            size.width * size.height
          }

          let monitor = window.current_monitor().unwrap();
          if let Some(mode) = monitor
            .video_modes()
            .max_by(|a, b| area(a.size()).cmp(&area(b.size())))
          {
            window.set_fullscreen(Some(Fullscreen::Exclusive(mode)));
          } else {
            eprintln!("no video modes available");
          }
        }
        "f" => {
          if window.fullscreen().is_some() {
            window.set_fullscreen(None);
          } else {
            let monitor = window.current_monitor();
            window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
          }
        }
        "p" => {
          if window.fullscreen().is_some() {
            window.set_fullscreen(None);
          } else {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
          }
        }
        "m" => {
          minimized = !minimized;
          window.set_minimized(minimized);
        }
        "q" => {
          *control_flow = ControlFlow::Exit;
        }
        "v" => {
          visible = !visible;
          window.set_visible(visible);
        }
        "x" => {
          let is_maximized = window.is_maximized();
          window.set_maximized(!is_maximized);
        }
        _ => (),
      },
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
        ..
      } if window_id == window.id() => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}
