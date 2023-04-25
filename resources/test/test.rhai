Main.console("start test.rhai");

fn onKeyPress() {
    Main.console("rhai! onKeyPress fired");
}
Main.register_event("onKeyPress", onKeyPress);
