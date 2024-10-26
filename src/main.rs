//Group 11
//team members:
//Ben Tran
//Daniel Leone
//Justin Halvorson

use rand::seq::SliceRandom;
use std::fs::{self};
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use std::io::{self};
use std::thread;
use filetime::FileTime;
use chrono::{DateTime, Local};

// Function to move a file to a target directory and rename it
fn move_and_rename_file(
    file_path: &std::path::Path,
    target_dir: &std::path::Path,
    new_name: &str,
) -> std::io::Result<PathBuf> {
    let mut new_path = target_dir.to_path_buf();
    new_path.push(new_name);
    fs::rename(file_path, &new_path)?;
    Ok(new_path)
}

// Function to display file details with error handling
fn display_file_details(file_path: &std::path::Path, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = fs::metadata(file_path)?;
    
    // Get both creation and modification times
    let created_time = metadata.created().ok();
    let modified_time = metadata.modified().ok();
    
    // Convert to local time for better readability
    let created_str = if let Some(time) = created_time {
        let local_time: DateTime<Local> = time.into();
        format!("Created: {}", local_time.format("%Y-%m-%d %H:%M:%S"))
    } else {
        "Created: Not available".to_string()
    };
    
    let modified_str = if let Some(time) = modified_time {
        let local_time: DateTime<Local> = time.into();
        format!("Modified: {}", local_time.format("%Y-%m-%d %H:%M:%S"))
    } else {
        "Modified: Not available".to_string()
    };
    
    println!("[{}] File: {:?}\n    {}\n    {}", label, file_path, created_str, modified_str);
    Ok(())
}

// Function to randomly rename files in a directory
fn rename_files_in_directory(dir: &std::path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        let mut files: Vec<_> = fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .collect();
        
        // Shuffle files to randomize the renaming
        let mut rng = rand::thread_rng();
        files.shuffle(&mut rng);

        let mut temp_dir_path = dir.to_path_buf();
        temp_dir_path.push("temp");
        let _ = fs::create_dir(temp_dir_path.as_path());

        for (index, entry) in files.iter().enumerate() {
            display_file_details(&entry.path(), "Before Rename").unwrap_or_else(|e| println!("Error displaying file details: {}", e));
            let file_name = format!("{:03}.csv", index + 1);
            let new_path = move_and_rename_file(&entry.path(), temp_dir_path.as_path(), &file_name)?;
            display_file_details(&new_path, "After Rename").unwrap_or_else(|e| println!("Error displaying file details: {}", e));
        }
        let _ = visit_sub_dirs(temp_dir_path.as_path(), dir);
        let _ = fs::remove_dir(temp_dir_path.as_path());
    }
    Ok(())
}

// Visit directories and process files
fn visit_dirs(dir: &std::path::Path) -> std::io::Result<()> {
    println!("Processing directories...");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Process only category directories
                if let Some(dir_name) = path.file_name() {
                    let dir_name = dir_name.to_string_lossy();
                    if ["takeoff", "land", "right", "left", "forward", "backward"].contains(&dir_name.as_ref()) {
                        println!("\nProcessing category: {}", dir_name);
                        visit_sub_dirs(&path, &path)?;
                        let _ = rename_files_in_directory(&path);
                    }
                }
            } else {
                println!("File: {:?}", path);
            }
        }
    }
    Ok(())
}

// Visit subdirectories, move files and change timestamps
fn visit_sub_dirs(dir: &std::path::Path, target_dir: &std::path::Path) -> std::io::Result<()> {
    println!("Processing subdirectories...");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_sub_dirs(&path, target_dir)?;
                let _ = fs::remove_dir(path);
            } else {
                display_file_details(&path, "Before Move").unwrap_or_else(|e| println!("Error displaying file details: {}", e));

                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                let new_path = move_and_rename_file(&path, target_dir, &file_name)?;

                display_file_details(&new_path, "After Move").unwrap_or_else(|e| println!("Error displaying file details: {}", e));

                change_file_timestamp(&new_path)?;
            }
        }
    }
    Ok(())
}

fn change_file_timestamp(file_path: &std::path::Path) -> std::io::Result<()> {
    // Set base time to September 22, 2023 00:00:00
    let seconds_since_epoch = 1695340800; // Unix timestamp for 2023-09-22 00:00:00
    let random_offset = rand::random::<u64>() % (10 * 24 * 60 * 60); // Random offset up to 10 days
    
    let new_time = SystemTime::UNIX_EPOCH + Duration::from_secs(seconds_since_epoch + random_offset);
    let file_time = FileTime::from_system_time(new_time);

    display_file_details(file_path, "Before Timestamp Change").unwrap_or_else(|e| println!("Error displaying file details: {}", e));

    filetime::set_file_mtime(file_path, file_time)?;

    display_file_details(file_path, "After Timestamp Change").unwrap_or_else(|e| println!("Error displaying file details: {}", e));
    Ok(())
}

// Function to run the tasks and automation options
fn run_tasks(main_directory: &std::path::Path, target_directory: &std::path::Path, interval_seconds: u64, do_loop: bool) -> std::io::Result<()> {
    loop {
        println!("\nNew run starting");
        println!("Processing directory: {:?}", main_directory);
       
        fs::create_dir_all(&target_directory)?;

        visit_dirs(&main_directory)?;

        println!("Completed run");
        println!("All automated tasks completed.");

        if !do_loop {
            break;
        }
        
        thread::sleep(Duration::from_secs(interval_seconds));
    }
    Ok(())
}

fn main() {
    println!("File Processing Program");
    println!("Authors: Justin Halvorson, Daniel Leone, Ben Tran");

    // Allow selection of path configuration
    println!("Select path configuration:");
    println!("1. Use default paths (C:\\Users\\bambo\\Downloads\\data\\data)");
    println!("2. Use alternate paths (C:\data)");
    println!("3. Enter custom paths");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let (main_directory, target_directory) = match input.trim() {
        "1" => (
            PathBuf::from(r"C:\Users\bambo\Downloads\data\data"),
            PathBuf::from(r"C:\Users\bambo\Downloads\data\data")
        ),
        "2" => (
            PathBuf::from(r"C:\data"),
            PathBuf::from(r"C:\data")
        ),
        "3" => {
            println!("Enter main directory path:");
            let mut path = String::new();
            io::stdin().read_line(&mut path).expect("Failed to read path");
            let main_dir = PathBuf::from(path.trim());
            
            println!("Enter target directory path (or press enter to use same as main):");
            let mut target_path = String::new();
            io::stdin().read_line(&mut target_path).expect("Failed to read path");
            let target_dir = if target_path.trim().is_empty() {
                main_dir.clone()
            } else {
                PathBuf::from(target_path.trim())
            };
            
            (main_dir, target_dir)
        },
        _ => {
            println!("Invalid input, using default paths");
            (
                PathBuf::from(r"C:\Users\bambo\Downloads\data\data"),
                PathBuf::from(r"C:\Users\bambo\Downloads\data\data")
            )
        }
    };

    println!("\nSelected paths:");
    println!("Main directory: {:?}", main_directory);
    println!("Target directory: {:?}", target_directory);

    println!("\nSelect operation mode:");
    println!("1. Manual shuffle");
    println!("2. Run every 30 seconds");
    println!("3. Run once a week");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Failed to parse number");
    
    match input {
        1 => run_tasks(&main_directory, &target_directory, 1, false).unwrap_or_else(|e| println!("Error: {}", e)),
        2 => run_tasks(&main_directory, &target_directory, 30, true).unwrap_or_else(|e| println!("Error: {}", e)),
        3 => run_tasks(&main_directory, &target_directory, 604800, true).unwrap_or_else(|e| println!("Error: {}", e)),
        _ => println!("Not a correct input"),
    }
}