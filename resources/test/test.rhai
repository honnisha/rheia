fn onKeyPress() {
    Main.console("onKeyPress fired");
}
Main.register_event("onKeyPress", onKeyPress);

fn onConsoleCommand(command) {
    Main.console(`onConsoleCommand fired; command:${command}`);
}
Main.register_event("onConsoleCommand", onConsoleCommand);

Main.console("test register");
