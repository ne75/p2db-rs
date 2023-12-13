use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::PopenError;

pub fn load_app(path: &String, port: &String) -> Result<i32, PopenError> {
    println!("load {} to {}", path, port);

    let args = [
        "/opt/p2llvm/bin/loadp2",
        "-ZERO",
        "-l",
        "2000000",
        "-v",
        "-FIFO",
        "4096",
        "-p",
        port,
        path
    ];

    let mut p = Popen::create(&args, PopenConfig{..Default::default()})?;

    p.wait()?;

    return Ok(0);
}