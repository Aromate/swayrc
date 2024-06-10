use std::io;

use cmd_lib::run_cmd;

pub fn creat_node(name: &str) -> io::Result<()> {
    let result = run_cmd!(
        swaymsg workspace ${name}
    );
    result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// same as creat_node
pub fn focus_node(name: &str) -> io::Result<()> {
    creat_node(name)
}

pub fn exec_binary(args: &[&str]) -> io::Result<()> {
    let args = args.join(" ");
    let result = run_cmd!(
        swaymsg exec ${args}
    );
    result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn swaymsg(args: &[&str]) -> io::Result<()> {
    let args = args.join(" ");
    let result = run_cmd!(
        swaymsg ${args}
    );
    result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
