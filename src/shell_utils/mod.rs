use std::process::Command;

pub trait CctkCommand {
    fn cctk_arg(&self) -> String;

    fn command(&self) -> Command {
        let mut command = Command::new("/usr/bin/cctk");
        command.arg(&self.cctk_arg());
        command
    }

    fn command_as_root(&self) -> Command {
        let mut command = ask_for_root();
        command.arg("/usr/bin/cctk").arg(&self.cctk_arg());
        command
    }
}

fn ask_for_root() -> Command {
    Command::new("/usr/bin/pkexec")
}
