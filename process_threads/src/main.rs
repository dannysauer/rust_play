use std::process::Command;

struct ProcessArgs<'pa> {
    cmd: &'pa str,
    args: Option<&'pa str>,
}

fn main() {
    let cmds = [
        ProcessArgs {
            cmd: "/bin/echo",
            args: Some("hello world"),
        },
        ProcessArgs {
            cmd: "/bin/echoo",
            args: Some("hello world"),
        },
        ProcessArgs {
            cmd: "/bin/date",
            args: None,
        },
        ProcessArgs {
            cmd: "/bin/sleep",
            args: inf,
        },
    ];

    t_child_launcher(&cmds[0]);
    t_child_launcher(&cmds[2]);
}

fn t_child_launcher(cmd_params: &ProcessArgs) {
    let mut cmd = Command::new(cmd_params.cmd);
    if cmd_params.args.is_some() {
        cmd.arg(cmd_params.args.unwrap());
    }
    let mut kid = cmd
        .spawn()
        .expect(&format!("failed to run '{}'", cmd_params.cmd));

    let ret = kid.wait().expect("failed to wait on kid");
    assert!(ret.success());
}
