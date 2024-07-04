#[cfg(target_os = "macos")]

pub mod macos {
  use cocoa::{
    appkit::{NSWindow, NSWindowCollectionBehavior},
    base::id,
    foundation::NSInteger,
  };
  use tauri::{Runtime, Window};

  pub trait WindowExtMacos {
    fn set_visisble_on_all_workspaces(&self);
  }

  impl<R: Runtime> WindowExtMacos for Window<R> {
    fn set_visisble_on_all_workspaces(&self) {
      const HIGHER_LEVEL_THAN_LEAGUE: NSInteger = 1001;
      unsafe {
        let ns_win = self.ns_window().unwrap() as id;

        ns_win.setLevel_(HIGHER_LEVEL_THAN_LEAGUE);
        ns_win.setCollectionBehavior_(
          NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
        );
      }
    }
  }
}
