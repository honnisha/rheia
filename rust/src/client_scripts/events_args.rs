use rhai::{FuncArgs, Dynamic};

#[derive(Debug)]
pub struct ConsoleEventArgs {
    message: String,
}

impl Clone for ConsoleEventArgs {
    fn clone(&self) -> ConsoleEventArgs {
        ConsoleEventArgs {
            message: self.message.clone(),
        }
    }
}

impl FuncArgs for ConsoleEventArgs {
    fn parse<ARGS: Extend<Dynamic>>(self, args: &mut ARGS) {
        args.extend(Some(self.message.into()));
    }
}
