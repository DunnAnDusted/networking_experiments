mod local_listener;
mod garage_listener;

use std::io;

pub const RUN: fn() -> io::Result<()> = FN_LIST[1];
const FN_LIST: [fn() -> io::Result<()>; 2] = [garage_listener::listen, local_listener::listen];
