use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

fn main() {
    // Create some argument vectors for lanuching external programs
    // let a = vec!["view", "-h", "/home/zeebrow/rust/stdin-stdout-adventures/file.bam"];
    // let outsam = vec!["view", "-bh", "-o", "/home/zeebrow/rust/stdin-stdout-adventures/rust.bam", "-"];
    let a = vec!["/home/zeebrow/sandcastle/aws-cost-calculator/did_i_run_this_today.py", "-t", "-f", "/home/zeebrow/sandcastle/aws-cost-calculator/.env"];
    let outsam = vec!["--"];

    let mut child = Command::new("/home/zeebrow/sandcastle/aws-cost-calculator/venv/bin/python3")
        .args(&a)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let outchild = Command::new("/usr/bin/cat")
        .args(&outsam)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    // Create a handle and writer for the stdin of the second process
    let mut outstdin = outchild.stdin.unwrap();
    let mut writer = BufWriter::new(&mut outstdin);

    // Loop over the output from the first process
    if let Some(ref mut stdout) = child.stdout {
        for line in BufReader::new(stdout).lines() {

            let mut l: String = line.unwrap();
            // Need to add an end of line character back to the string
            // let eol: &str = "\n";
            // l = l + eol;
            // let color = "\x1b[38;2;100;100;100m";
            use std::str;
            l = str::replace(&l, "\t", "    ");
            let color = "\x1b[48;2;100;100;100m";
            let reset = "\x1b[0m\n";
            

            // Print some select lines from the first child to stdin of second
            if (l.chars().skip(0).next().unwrap()) != '-' {
            // if (l.chars().nth(0).unwrap()) != '-' {
                // convert the string into bytes and write to second process
                l = color.to_owned() + &l;
                l = l + reset;
                let bytestring = l.as_bytes();
                writer.write_all(bytestring).unwrap();
            } else {
                let eol: &str = "\n";
                l = l + eol;
                let bytestring = l.as_bytes();
                writer.write_all(bytestring).unwrap();
            }
        }
    }
}
