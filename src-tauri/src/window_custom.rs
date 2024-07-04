#[cfg(target_os = "macos")]

pub mod macos {
  use cocoa::{
    appkit::{NSWindow, NSWindowCollectionBehavior},
    base::id,
  };
  use tauri::{Runtime, Window};

  pub trait WindowExtMacos {
    fn set_visisble_on_all_workspaces(&self);
  }

  impl<R: Runtime> WindowExtMacos for Window<R> {
    fn set_visisble_on_all_workspaces(&self) {
      unsafe {
        let ns_win = self.ns_window().unwrap() as id;

        ns_win.setCollectionBehavior_(
          NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
        );
      }
    }
  }
}
