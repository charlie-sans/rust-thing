use std::io;

fn main() {
    print!("welcome to the rust git api command prompt\n it's a work in progress\n please enter a command\n profiles\n repos\n commits\n languages\n contributors\n");
    let mut gitapikey = String::new();
    let mut organisation = String::new();
    let mut repo = String::new();
    let mut commit = String::new();
    let mut language = String::new();
    let mut contributor = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if input.trim() == "profiles" {
        // run a python file while passing in the gitapikey
        println!("Please enter your git api key");
        io::stdin().read_line(&mut gitapikey).unwrap();
        print!("Please enter the organisation you want to search for");
        io::stdin().read_line(&mut organisation).unwrap();

        let output = std::process::Command::new("python")
            .arg("src/profiles.py")
            .arg(gitapikey.trim())
            .arg(organisation.trim())

            .output()
            .expect("failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    } else if input.trim() == "repos" {
        // run a python file while passing in the gitapikey// also repeating the same code for the other else if statements
        io::stdin().read_line(&mut organisation).unwrap();
        let output = std::process::Command::new("python")
           
            .arg("src/repo.py")
            .arg(organisation.trim())
            .output()
            .expect("failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }else if input.trim() == "commits" {
        // run a python file while passing in the gitapikey// also repeating the same code for the other else if statements
        println!("Please enter your git api key");
        io::stdin().read_line(&mut gitapikey).unwrap();

        let output = std::process::Command::new("python")
            .arg("src/commits.py")
            .arg(gitapikey.trim())
            .output()
            .expect("failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }else if input.trim() == "profiles" {
        // run a python file while passing in the gitapikey// also repeating the same code for the other else if statements
        println!("Please enter your git api key");
        io::stdin().read_line(&mut gitapikey).unwrap();

        let output = std::process::Command::new("python")
            .arg("src/profiles.py")
            .arg(gitapikey.trim())
            .output()
            .expect("failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    else if input.trim() == "projects" {
        // run a python file while passing in the gitapikey// also repeating the same code for the other else if statements
        
        println!("Please enter the organisation you want to search for");
        io::stdin().read_line(&mut organisation).unwrap();

        let output = std::process::Command::new("python")
            .arg("src/projects.py")
            
            .arg(organisation.trim())
            .output()
            .expect("failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
        
     else {
        println!("I didn't understand that.");
        main()
    }
}
