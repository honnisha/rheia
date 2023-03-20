fn onKeyPress() {
    godot_print("onKeyPress fired");
}

godot_print("test register");
registerEvent("onKeyPress", onKeyPress);
