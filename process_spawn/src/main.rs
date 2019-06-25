use std::process::Command;

fn main() {
  let cmd  = "/bin/echo";
  let cmd2 = "/bin/echoo";
  let mut kid = Command::new(cmd)
                        .arg("hello world")
                        .spawn()
                        .expect(&format!("failed to run '{}'", cmd));
  let ret = kid.wait()
               .expect("failed to wait on kid");
  assert!(ret.success());

  kid = Command::new(cmd2)
                        .arg("hello world")
                        .spawn()
                        .expect(&format!("failed to run '{}'", cmd2));
  let ret2 = kid.wait()
               .expect("failed to wait on kid");

  assert!(ret2.success());
}
