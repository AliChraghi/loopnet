use std::{
    array::IntoIter,
    collections::HashMap,
    io::{ErrorKind, Error},
    iter::FromIterator,
};

pub fn help_text(err: Error) -> &'static str {
    let tips = HashMap::<ErrorKind, &str>::from_iter(IntoIter::new([
        (
            ErrorKind::PermissionDenied,
            "Run App as Root (Run as Administrator In Windows & sudo in UNIX Like)"
        ),
        (
            ErrorKind::ConnectionRefused,
            "The port is not open on the your machine, the pending connections is full OR a firewall blocking access"
        ),
        (
            ErrorKind::ConnectionReset,
            "There is currently no solution. create an issue at https://github.com/AliChraghi/hoop/issues/new"
        ),
        (
            ErrorKind::ConnectionAborted,
            "There is currently no solution. create an issue at https://github.com/AliChraghi/hoop/issues/new"
        ),
        (
            ErrorKind::NotConnected,
            "restart your pc if you are on macOS or else create an issue at https://github.com/AliChraghi/hoop/issues/new"
        ),
        (
            ErrorKind::AddrInUse,
            "Kill the process that uses address"
        ),
        (
            ErrorKind::AddrNotAvailable,
            "the requested address was not local. run at a local address"
        ),
        (
            ErrorKind::BrokenPipe,
            "There is currently no solution. create an issue at https://github.com/AliChraghi/hoop/issues/new"
        ),
    ]));

    loop {
        for (k, v) in &tips {
            if k == &err.kind() {
                return v;
            }
        }
    }
}
