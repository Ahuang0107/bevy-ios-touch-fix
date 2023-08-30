use bevy_app::{App, Plugin};
use bevy_ecs::prelude::Resource;
use bevy_math::Vec2;

/// fix the screen size when running on the ios device.
///
/// in bevy, the touch position is not correct when app running on ios real device( it work fine on simulation).
///
/// so you need use correct screen size to fix the touch position.
///
/// ``` rust
/// # use std::ops::Deref;
/// # use bevy_ecs::prelude::{EventReader, Query, Res};
/// # use bevy_input::prelude::TouchInput;
/// # use bevy_math::Vec2;
/// # use bevy_window::Window;
/// # use bevy_ios_touch_fix::ScreenFixedSize;
/// pub fn handle_touch(
///     windows: Query<&Window>,
///     screen_fixed_size: Res<ScreenFixedSize>,
///     mut touch_evs: EventReader<TouchInput>,
/// ){
///     let window = windows.get_single().unwrap();
///     let window_size = Vec2::new(window.width(), window.height());
///     for touch_ev in touch_evs.iter() {
///         let fixed_position =
///             if let Some(screen_fixed_size) = screen_fixed_size.size {
///                 (touch_ev.position / screen_fixed_size) * window_size
///             } else {
///                 touch_ev.position
///             };
///         // now you have get the correct touch position
///     }
/// }
/// ```
pub struct ScreenSizeFixPlugin;

/// Actual Screen Size from Apple UiKit UIScreen
#[derive(Resource)]
pub struct ScreenFixedSize {
    pub size: Option<Vec2>,
}

impl Plugin for ScreenSizeFixPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_os = "ios"))]
        let window_size = None;
        #[cfg(target_os = "ios")]
        let window_size = unsafe {
            use objc2::msg_send;
            use objc2::runtime::{AnyClass, AnyObject};

            let ui_screen_class = AnyClass::get("UIScreen").unwrap();
            let main_screen: *mut AnyObject = msg_send![ui_screen_class, mainScreen];
            let bounds: core_graphics::CGRect = msg_send![main_screen, bounds];
            let width: f32 = bounds.size.width as f32;
            let height: f32 = bounds.size.height as f32;
            Some(Vec2::new(width, height))
        };
        app.insert_resource(ScreenFixedSize { size: window_size });
    }
}

#[cfg(target_os = "ios")]
mod core_graphics {
    use objc2::{Encode, Encoding};

    #[cfg(target_pointer_width = "32")]
    type CGFloat = f32;
    #[cfg(target_pointer_width = "64")]
    type CGFloat = f64;

    #[repr(C)]
    struct CGPoint {
        x: CGFloat,
        y: CGFloat,
    }

    // SAFETY: The struct is `repr(C)`, and the encoding is correct.
    unsafe impl Encode for CGPoint {
        const ENCODING: Encoding =
            Encoding::Struct("CGPoint", &[CGFloat::ENCODING, CGFloat::ENCODING]);
    }

    #[repr(C)]
    pub struct CGSize {
        pub width: CGFloat,
        pub height: CGFloat,
    }

    // SAFETY: The struct is `repr(C)`, and the encoding is correct.
    unsafe impl Encode for CGSize {
        const ENCODING: Encoding =
            Encoding::Struct("CGSize", &[CGFloat::ENCODING, CGFloat::ENCODING]);
    }

    #[repr(C)]
    pub struct CGRect {
        origin: CGPoint,
        pub size: CGSize,
    }

    // SAFETY: The struct is `repr(C)`, and the encoding is correct.
    unsafe impl Encode for CGRect {
        const ENCODING: Encoding =
            Encoding::Struct("CGRect", &[CGPoint::ENCODING, CGSize::ENCODING]);
    }
}
