use clap::Parser;

#[derive(Parser)]
#[command(name = "HwsMaster for linux")]
#[command(author = "Jerry Wang. <freebufer.wang@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "HwsMaster for linux
Written in rust, it has unparalleled performance and security
making it very easy to build software under Linux", long_about = None)]
pub struct Cli {}
