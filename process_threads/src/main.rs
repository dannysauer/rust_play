use std::process::Command;
use std::thread;

struct ProcessArgs<'pa> {
    cmd: &'pa str,
    args: Option<&'pa str>, // this should be an iterable
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
            args: Some("inf"),
        },
    ];

    // child_launcher(&cmds[0]);
    // child_launcher(&cmds[2]);

    let runner = thread::spawn(move || {
        t_process_runner(4, &cmds[3]);
    });
    runner.join().unwrap_or_else(|e| {
        panic!("Some kind of error happened: {:?}", e);
    });
}

/* there needs to be a way to change the number
   Perhaps use a struct with a change_count method and the array of runners?
   Pass the struct into the thread, and mutate the struct?
*/
fn t_process_runner(count: usize, args: &ProcessArgs) {
    let mut running = Vec::with_capacity(count);
    let mut dead = Vec::new();

    loop {
        // increase running instances as needed
        while running.len() < count {
            // there needs to be a way to get the places to run; add to launcher
            //running.push(&child_launcher(args));
            let kid = child_launcher(args);
            println!("Spawned process {}", kid.id());
            running.push(kid);
        }

        // check on dead kids
        for i in 0..running.len() {
            // let mut kid = running[i];
            match running[i].try_wait() {
                Ok(Some(status)) => {
                    println!("Process {} exited w/ {}", running[i].id(), status);
                    dead.push(i);
                }
                Ok(None) => {
                    //println!("Process {} is ok", running[i].id());
                }
                Err(e) => {
                    println!("Failed checking process: {}", e);
                    dead.push(i);
                }
            };
        }

        // remove after iterating so we don't screw up vector length
        if dead.len() > 0 {
            // remove biggest to smallest index
            dead.reverse();
            for d in &dead {
                running.remove(*d);
            }
            dead.clear();
        }

        /* // this'd be great
        running.retain(|&mut kid| {
            match kid.try_wait() {
                Ok(Some(status)) => return true,
                Ok(None) => return false,
                Err(e) => return true,
            }
        })
        */
    }
}

// this could easily be "run a thing on a machine" with another param
fn child_launcher(cmd_params: &ProcessArgs) -> std::process::Child {
    let mut cmd = Command::new(cmd_params.cmd);
    if cmd_params.args.is_some() {
        cmd.arg(cmd_params.args.unwrap());
    }
    let kid = cmd
        .spawn()
        .expect(&format!("failed to run '{}'", cmd_params.cmd));

    return kid;
}
