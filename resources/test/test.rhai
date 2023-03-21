fn onKeyPress() {
    godot_print("onKeyPress fired");
}
registerEvent("onKeyPress", onKeyPress);

godot_print("test register");
