// functions that will run for a specific confguration option
use dialoguer::Input;

pub fn conf_max_concurrent() {
    // confgures how many instances of a project's application can run at once (usually limited by CPU codes)
    let boinc_path: u16 = Input:new()
            .with_prompt("What is the MAXIMUM amount of instances do you want of this project running? ")
            .interact_text()
            .unwrap();

    println!(boinc_path.to_string());
    return boinc_path;
}
