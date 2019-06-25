use std::process::Command;

fn main() {
  let cmds = ["/bin/echo",
    "/bin/echoo",
    "/bin/date",
    ];

  t_cmd_launcher(cmds[0], "hello world");
}

fn t_cmd_launcher(cmd:&str, arg:&str) {
  let mut kid = Command::new(cmd)
                        .arg(arg)
                        .spawn()
                        .expect(&format!("failed to run '{}'", cmd));
  let ret = kid.wait()
               .expect("failed to wait on kid");
  assert!(ret.success());
}
