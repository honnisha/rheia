Main.console("start test.rhai");

fn onKeyPress() {
    Main.console("rhai! onKeyPress fired");
}
Main.register_event("onKeyPress", onKeyPress);

let test_command = #{
  name: "test",
  subcommands: [
    #{
      name: "first",
      short_flag: "f",
      args: [
        #{
          name: "test arg",
        },
      ]
    },
  ]
};
Main.register_command(commandHandler, test_command);
fn commandHandler(command) {
    Main.console(`rhai! fired; command:${command}`);
}

Main.console("test register");
