use druid::widget::{Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

mod thread;
pub use crate::thread::*;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Clone, Data, Lens)]
pub struct HelloState {
  name: i32,
}

// #[no_mangle]
// pub unsafe extern "C" fn _AppLauncher_launch(al: *const AppLauncher<HelloState>, data: HelloState) {
//   let _ = AppLauncher::launch(std::ptr::read(al), data);
// }

// #[no_mangle]
// pub unsafe extern "C" fn _AppLauncher_withWindow(
//   window: *const WindowDesc<HelloState>,
// ) -> *const AppLauncher<HelloState> {
//   let al = AppLauncher::with_window(ptr::read(window));
//   let ret = al as *const AppLauncher<HelloState>;
//   mem::forget(al);
//   ret
// }

// #[no_mangle]
// pub unsafe extern "C" fn _WindowDesc_new(root) -> *const WindowDesc {
//   WindowDesc::new()
// }
