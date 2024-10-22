use rand::seq::SliceRandom;
use std::fs::{self};
use std::path;
use std::time::{SystemTime, UNIX_EPOCH, Duration, SystemTimeError};
use std::io::{self};
use std::{thread, time::Instant};
use filetime::FileTime;

// Function to move a file to a target directory and rename it
fn move_and_rename_file(
    file_path: &path::Path,
    target_dir: &path::Path,
    new_name: &str,
) -> std::io::Result<path::PathBuf> {
    let mut new_path = target_dir.to_path_buf();
    new_path.push(new_name);
    fs::rename(file_path, &new_path)?;
    Ok(new_path)
}

// Function to display file details with error handling
fn display_file_details(file_path: &path::Path, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = fs::metadata(file_path)?;
    let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
    println!("[{}] File: {:?}, Modified Timestamp: {}", label, file_path, modified_time);
    Ok(())
}

// Function to randomly rename files in a directory
fn rename_files_in_directory(dir: &path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        let mut files: Vec<_> = fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .collect();
        
        // Shuffle files to randomize the renaming
        let mut rng = rand::thread_rng();
        files.shuffle(&mut rng);

        for (index, entry) in files.iter().enumerate() {
            display_file_details(&entry.path(), "Before Rename").unwrap();
            let file_name = format!("{:03}.jpg", index + 1); // Adjust file extension as needed
            let new_path = move_and_rename_file(&entry.path(), dir, &file_name)?;
            display_file_details(&new_path, "After Rename").unwrap();
        }
    }
    Ok(())
}

// Visit directories and process files
fn visit_dirs(dir: &path::Path, target_dir: &path::Path) -> std::io::Result<()> {
    println!("Processing directories...");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Recursively visit subdirectories
                visit_sub_dirs(&path, target_dir)?;
            } else {
                println!("File: {:?}", path);
            }
        }
    }
    Ok(())
}

// Visit subdirectories, move files and change timestamps
fn visit_sub_dirs(dir: &path::Path, target_dir: &path::Path) -> std::io::Result<()> {
    println!("Processing subdirectories...");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_sub_dirs(&path, target_dir)?;
            } else {
                // Display file details before move
                display_file_details(&path, "Before Move").unwrap();

                // Move and rename file
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                let new_path = move_and_rename_file(&path, target_dir, &file_name)?;

                // Display file details
                display_file_details(&new_path, "After Move").unwrap();

                // Change file timestamp to a random date in the last 10 days
                change_file_timestamp(&new_path)?;
            }
        }
    }
    Ok(())
}

// Function to change the timestamp of a file
fn change_file_timestamp(file_path: &path::Path) -> std::io::Result<()> {
    let now = SystemTime::now();
    let random_offset = Duration::from_secs(rand::random::<u64>() % (10 * 24 * 60 * 60)); // Up to 10 days
    let new_time = now - random_offset;

    // Display details before the timestamp
    display_file_details(file_path, "Before Timestamp Change").unwrap();

    // Change file's timestamp
    filetime::set_file_mtime(file_path, FileTime::from_system_time(new_time))?;

    // Display details after the timestamp
    display_file_details(file_path, "After Timestamp Change").unwrap();
    Ok(())
}

// Function to run the tasks and automation options
fn run_tasks(main_directory: &path::Path, target_directory: &path::Path, interval_seconds: u64, times: u32) -> std::io::Result<()> {
    for run in 1..=times {
        println!("--- Run {}/{} ---", run, times);
        let start_time = Instant::now();

        fs::create_dir_all(&target_directory)?;

        // Visit directories and move files
        visit_dirs(&main_directory, &target_directory)?;

        // Rename files in target directory
        rename_files_in_directory(&target_directory)?;

        println!("Completed run {}/{}. Duration: {:?}", run, times, start_time.elapsed());
        
        // Sleep for the interval unless last run
        if run < times {
            thread::sleep(Duration::from_secs(interval_seconds));
        }
    }

    println!("All automated tasks completed.");
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Use double backslashes for Windows paths or raw strings (r"")
    let main_directory = path::PathBuf::from(r"C:\Users\cheet\data");
    let target_directory = path::PathBuf::from(r"C:\Users\cheet\data");

    // Set the interval (in seconds) and number of automatic runs
    let interval_seconds = 30; // Change to 604800 for a weekly interval (60 * 60 * 24 * 7)
    let number_of_runs = 3;

    run_tasks(&main_directory, &target_directory, interval_seconds, number_of_runs)
}
