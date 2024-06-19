use std::process::Command;
use std::io::Write;
use std::io::Read;

pub fn test(){
    //spawnner();
    test_stdin();
}

fn test_stdin() {
    let mut head_cmd = Command::new("head");
    head_cmd.arg("-n 1");
    head_cmd.stdin(std::process::Stdio::piped());
    head_cmd.stdout(std::process::Stdio::piped());

    let input_data = "inputone\ninputtwo".as_bytes();

    let mut proc_handle = head_cmd.spawn().unwrap();
    let mut stdin_handle = proc_handle.stdin.take().unwrap();

    _ = stdin_handle.write_all(input_data);
    let x = proc_handle.wait();
    let mut output_buffer = String::new();
    //if stdout is not piped() then it throws error with stddout.unwrap() -> giving none 
    let stdout_result = proc_handle.stdout.unwrap().read_to_string(&mut output_buffer);

    println!("Result was: {}", output_buffer);

}

fn spawnner() {
    let mut p1 = Command::new("which");
    p1.arg("python");
    let proc_result = p1.output();
    if proc_result.is_ok() {
        let result = proc_result.ok().unwrap();
        println!("Was execution successful? {}", result.status.success());
        if !result.status.success() {
            println!("Error occurred: {}", result.status.code().unwrap());
        }
        else {
            println!("{:#?}", String::from_utf8(result.stdout).unwrap());
        }
    }

    let mut p2 = Command::new("pwsh");
    p2.args([".Command", "Write-Host", "Hello from PowerShell"]);
    //prevent program from outputting
    p2.stdout(std::process::Stdio::null());
    let mut p2_handle = p2.spawn().unwrap();

    println!("DOing some more work...");
    let proc_result = p2_handle.wait().unwrap();
    println!("Exited with code: {}", proc_result.code().unwrap());

}