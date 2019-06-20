#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd",
           target_os = "netbsd", target_os = "openbsd"))]

pub use self::window::Window;
pub use self::event_loop::{EventLoop, EventLoopWindowTarget, EventLoopProxy, WindowEventsSink, MonitorHandle};

use smithay_client_toolkit::reexports::client::protocol::wl_surface;

mod event_loop;
mod pointer;
mod touch;
mod keyboard;
mod window;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId;

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        DeviceId
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(usize);

impl WindowId {
    pub unsafe fn dummy() -> Self {
        WindowId(0)
    }
}

#[inline]
fn make_wid(s: &wl_surface::WlSurface) -> WindowId {
    WindowId(s.as_ref().c_ptr() as usize)
}
