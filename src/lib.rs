mod garage_listener;

use std::io;

pub const RUN: fn() -> io::Result<()> = FN_LIST[0];
const FN_LIST: [fn() -> io::Result<()>; 1] = [garage_listener::listen];