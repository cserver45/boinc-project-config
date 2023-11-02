use std::fs;

use dialoguer::Input;
use dialoguer::Confirm;

fn main() {
    // first, ask the user if BOINC is installed in a custom directory
    let boinc_dir_option = Confirm::new()
            .with_prompt("Did you install BOINC somewhere than the default directory? (Default is /var/lib/boinc/projects): ")
            .default(false)
            .interact()
            .unwrap();

    if boinc_dir_option {
        let boinc_path = Input:new()
            .with_prompt("Ok then, put your FULL directory path here to the projects: ")
            .interact_text()
            .unwrap();
    } else {
        let boinc_path = "/var/lib/boinc/projects"
        println!("Ok then, going with the default diretory (/var/lib/boinc/projects)")
    }
    //let project_name = "someproject.com_project";


    // lets give the user options as to what projects are available
    let paths = fs::read_dir(boinc_path).unwrap();

    let selection = Select::new()
        .with_prompt("Chose a project: ")
        .items(&paths)
        .interact()
        .unwrap();
    

    // now we need options for the user, from this documentation: https://boinc.berkeley.edu/wiki/Client_configuration
    // ill probably make this some kind of function, so I don't have 30 different if trees or a large match tree

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
