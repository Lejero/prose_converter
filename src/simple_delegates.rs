use druid::{AppDelegate, Data, DelegateCtx, Env, WindowId};

pub struct TerminateOnCloseDelegate {}

impl<T: Data> AppDelegate<T> for TerminateOnCloseDelegate {
    fn window_removed(&mut self, _id: WindowId, _data: &mut T, _env: &Env, _ctx: &mut DelegateCtx) {
        std::process::exit(0x0000);
    }
}
