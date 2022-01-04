use std::thread::{self, Thread, ThreadId};

#[no_mangle]
pub unsafe extern "C" fn rstd_thread_Thread__drop(thread: usize) {
  let pointer = thread as *mut Thread;
  // Reclaim memory and let Rust dispose of it
  let _ = std::ptr::read(pointer);
}

#[no_mangle]
pub unsafe extern "C" fn rstd_thread_current() -> usize {
  let mut thread = thread::current();
  let ptr = &mut thread as *mut Thread;
  // Prevent Rust freeing memory at end of function
  std::mem::forget(thread);
  ptr as usize
}

#[no_mangle]
pub unsafe extern "C" fn rstd_thread_Thread_id(thread: usize) -> usize {
  let pointer = thread as *mut Thread;
  let threadobj = &mut *pointer;
  let tid = &mut threadobj.id();
  let ret = tid as *mut ThreadId as usize;
  ret
}

#[no_mangle]
pub unsafe extern "C" fn lol_rstd_thread_current() -> Thread {
  thread::current()
}

#[no_mangle]
pub trait ATrait {
  fn afn(self: &Self);
}

#[no_mangle]
pub unsafe extern "C" fn _test(x: dyn ATrait) {}
