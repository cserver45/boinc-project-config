use std::fs;

use dialoguer::Confirm;
use dialoguer::Input;
use dialoguer::Select;


fn main() {
    // first, ask the user if BOINC is installed in a custom directory
    let boinc_dir_option = Confirm::new()
            .with_prompt("Did you install BOINC somewhere than the default directory? (Default is /var/lib/boinc/projects): ")
            .default(false)
            .interact()
            .unwrap();

    let mut boinc_path = String::new();
    if boinc_dir_option {
        boinc_path = Input::new()
            .with_prompt("Ok then, put your FULL directory path here to the projects: ")
            .interact_text()
            .unwrap();
    } else {
        //println!("Ok then, going with the default diretory (/var/lib/boinc/projects)");
        boinc_path = "/var/lib/boinc/projects".to_string();
    }
    let project_name = "someproject.com_project";

    // lets give the user options as to what projects are available
    let paths = fs::read_dir(boinc_path).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }

    //let project_name = Select::new()
    //    .with_prompt("Chose a project: ")
    //    .items(&paths)
    //    .interact()
    //    .unwrap();

    // now we need options for the user, from this documentation: https://boinc.berkeley.edu/wiki/Client_configuration
    // ill probably make this some kind of function, so I don't have 30 different if trees or a large match tree
    let options = menu_top(format!("{project_name}/app_config.xml"));

    // compile into one list after the user has told us to save
    let options = "some string hm";

    // directory path, by default it stores it in /var/lib/boinc/projects/<project name>
    fs::write(format!("{project_name}/app_config.xml"), options);

    let restart_option = Confirm::new()
        .with_prompt("Wrote file sucessfully, would you like to restart BOINC?")
        .default(true)
        .interact()
        .unwrap();

    if restart_option {
        println!("Restarting BOINC...");
        // and then restart the service somehow
    } else {
        println!("You will need to restart BOINC yourself then. Goodbye!");
    }
}

fn menu_top(project: String) {
    // the main selection menu
    let mut editing = true;
    while editing == true {
        println!("running the loop");
    }
}
