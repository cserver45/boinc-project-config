use std::fs;

//use dialoguer::Input;
use dialoguer::Confirm;

fn main() {
    // first, we need a menu system for options
    let project_name = "someproject.com_project";

    // compile into one list after the user has told us to save
    let options = "some string hm";

    // write to the directory
    println!("Hello world");
    // directory path, by default it stores it in /var/lib/boinc/projects/<project name>
    fs::write(format!("{project_name}/app_config.xml"), options);

    let restart_option = Confirm::new()
            .with_prompt("Wrote file sucessfully, would you like to restart BOINC?")
            .default(true)
            .interact()
            .unwrap();

    if restart_option {
        println!("Restarting BOINC...")
        // and then restart the service somehow
    } else {
        println!("You will need to restart BOINC yourself then. Goodbye!")
    }

}
