mod scheduler;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use scheduler::{Task, fcfs, round_robin, calculate_metrics};

// Read tasks from input.txt file
fn read_tasks_from_file(file_path: &str) -> io::Result<Vec<Task>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let tasks = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let parts: Vec<u32> = line.split_whitespace()
                                      .filter_map(|word| word.parse::<u32>().ok())
                                      .collect();
            if parts.len() == 3 {
                Some(Task {
                    pid: parts[0],
                    time_arrival: parts[1],
                    burst_time: parts[2],
                    remaining_time: parts[2],
                    start_time: None,
                    finish_time: None,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(tasks)
}

// Main function
fn main() {
    let tasks_file_path = "input.txt";
    match read_tasks_from_file(tasks_file_path) {
        Ok(tasks) => {

            let completed_fcfs = fcfs(tasks.clone());
            let (fcfs_avg_wait, fcfs_avg_turn) = calculate_metrics(&completed_fcfs);
            println!("FCFS Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", fcfs_avg_wait, fcfs_avg_turn);

            let completed_rr = round_robin(tasks.clone(), 2);
            let (rr_avg_wait, rr_avg_turn) = calculate_metrics(&completed_rr);
            println!("RR Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", rr_avg_wait, rr_avg_turn);
        },
        Err(e) => {
            eprintln!("Failed to read tasks from {}: {}", tasks_file_path, e);
        }
    }
}
