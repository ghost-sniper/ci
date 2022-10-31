use std::process;
use std::process::Command;

fn ps() {
    let output = Command::new("ps").arg("-q").arg(process::id().to_string()).arg("-o").arg("%cpu, %mem").output().unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);
}

mod test {
    use crate::execute::ps;

    #[test]
    fn test_command() {
        ps();
    }
}