#[cfg(target_os = "macos")]

pub mod macos {
  use cocoa::{
    appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior},
    base::id,
    foundation::NSInteger,
  };
  use tauri::{Runtime, Window};

  pub trait WindowExtMacos {
    fn set_visisble_on_all_workspaces(&self, enabled: bool);
  }

  impl<R: Runtime> WindowExtMacos for Window<R> {
    fn set_visisble_on_all_workspaces(&self, enabled: bool) {
      const HIGHER_LEVEL_THAN_LEAGUE: NSInteger = 1001;
      unsafe {
        let ns_win = self.ns_window().unwrap() as id;

        if enabled {
          ns_win.setLevel_(HIGHER_LEVEL_THAN_LEAGUE);
          ns_win.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
          );
        } else {
          ns_win.setLevel_(((NSMainMenuWindowLevel - 1) as u64).try_into().unwrap());
          ns_win
            .setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorDefault);
        }
      }
    }
  }
}
