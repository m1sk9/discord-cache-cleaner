use directories::ProjectDirs;
use std::fs;

pub fn clean_cache() {
    println!("Searching for Application Support directory...");
    if let Some(discord_instance_dir) = ProjectDirs::from("", "", "discord") {
        let application_support_dir = discord_instance_dir.config_dir();
        println!("Found Application Support directory: {:?}", application_support_dir);

        println!("Searching for Discord cache directory...");

        let mut discord_cache_dirs = vec![
            application_support_dir.join("Cache"),
            application_support_dir.join("Code Cache"),
            application_support_dir.join("GPUCache"),
            application_support_dir.join("DawnCache"),
        ];

        for dir in discord_cache_dirs.iter_mut() {
            if dir.exists() {
                println!("Found Discord cache directory: {:?}", dir);
                fs::remove_dir_all(dir).expect("Could not remove Discord cache directory!");
                println!("Successfully cleaned Discord cache directory!")
            } else {
                println!("Could not find Discord cache directory: {:?}", dir);
            }
        }

        println!("Successfully cleaned Discord all cache directory! Restart Discord instance!");
    } else {
        println!("Could not find Application Support directory.")
    }
}
