# readme

when running bevy app on real ios device, the touch event position is not correct, this plugin provide a workaround to
get correct touch position.

```rust
use std::ops::Deref;
use bevy::prelude::*;
use bevy_ios_touch_fix::{ScreenFixedSize, ScreenSizeFixPlugin};

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ScreenSizeFixPlugin)
        .add_system(handle_touch)
        .run();
}

fn handle_touch(
    windows: Query<&Window>,
    screen_fixed_size: Res<ScreenFixedSize>,
    mut touch_evs: EventReader<TouchInput>,
) {
    let window = windows.get_single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    for touch_ev in touch_evs.iter() {
        let fixed_position =
            if let Some(screen_fixed_size) = screen_fixed_size.size {
                (touch_ev.position / screen_fixed_size) * window_size
            } else {
                touch_ev.position
            };
        // now you have get the correct touch position
    }
}
```