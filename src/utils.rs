pub fn add_command_args<T: Into<Command>>(&mut self, value: T) -> &mut Self {
    let command: Command = value.into();
    self.args(command.get_args());
    self
}