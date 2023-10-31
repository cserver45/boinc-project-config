use std::fs;

fn main() {
    // first, we need a menu system for options
    let project_name = "someproject.com_project";

    // compile into one list after the user has told us to save
    let options = "some string hm";

    // write to the directory
    println!("Hello world");
    // directory path, by default it stores it in /var/lib/boinc/projects/<project name>
    let result_w = fs::write(format!("{project_name}/app_config.xml"), options);

    match result_w {
        Ok(_) => print!("Wrote file sucessfully, would you like to restart BOINC? [Y/n]: "),
        Err(error) => panic!(
            "Seems like something went wrong when writing the file: {:?}",
            error
        ),
    };
}
