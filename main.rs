mod scheduler;
use scheduler::{calculate_metrics, fcfs, round_robin, sjf, spn, srtf,priority, Task};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Read tasks from input.txt file
fn read_tasks_from_file(file_path: &str) -> io::Result<Vec<Task>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let tasks = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let parts: Vec<u32> = line
                .split_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();
            if parts.len() == 4 {
                Some(Task {
                    pid: parts[0],
                    time_arrival: parts[1],
                    burst_time: parts[2],
                    remaining_time: parts[2],
                    start_time: None,
                    finish_time: None,
                    priority: parts[3],
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

            println!("SPN (Shortest Process Next) Output:");
            let completed_spn = spn(tasks.clone());
            let (spn_avg_wait, spn_avg_turn) = calculate_metrics(&completed_spn);
            println!("SPN Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", spn_avg_wait, spn_avg_turn);

            let completed_sjf = sjf(tasks.clone());
            let (sjf_avg_wait, sjf_avg_turn) = calculate_metrics(&completed_sjf);
            println!("SJF Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", sjf_avg_wait, sjf_avg_turn);

            let completed_srtf = srtf(tasks.clone());
            let (srtf_avg_wait, srtf_avg_turn) = calculate_metrics(&completed_srtf);
            println!(
                "SRTF Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}",
                srtf_avg_wait, srtf_avg_turn
            );

            let completed_priority = priority(tasks.clone());
            let (priority_avg_wait, priority_avg_turn) = calculate_metrics(&completed_priority);
            println!(
                "Priority Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}",
                priority_avg_wait, priority_avg_turn
            );

        }
        Err(e) => {
            eprintln!("Failed to read tasks from {}: {}", tasks_file_path, e);
        }
    }
}
