#[derive(Clone)]
struct Task {
    pid: u32,  // Process ID
    time_arrival: u32,  // Arrival time of the task
    burst_time: u32,  // CPU burst time required by the task
    remaining_time: u32,  // Time remaining for the task completion (used in SRTF and RR)
    start_time: Option<u32>,  // Start time of the task in CPU
    finish_time: Option<u32>,  // Finish time of the task from CPU
}

// FCFS Scheduler Function
fn fcfs(mut tasks: Vec<Task>) -> Vec<Task> {
    tasks.sort_by_key(|task| task.time_arrival);
    let mut current_time = 0;
    for task in tasks.iter_mut() {
        if current_time < task.time_arrival {
            current_time = task.time_arrival;
        }
        task.start_time = Some(current_time);
        current_time += task.burst_time;
        task.finish_time = Some(current_time);
    }
    tasks
}

// Function to calculate average waiting time and turnaround time
fn calculate_metrics(tasks: &[Task]) -> (f64, f64) {
    let total_waiting_time: u32 = tasks.iter().map(|task| task.finish_time.unwrap() - task.time_arrival - task.burst_time).sum();
    let total_turnaround_time: u32 = tasks.iter().map(|task| task.finish_time.unwrap() - task.time_arrival).sum();

    let average_waiting_time = total_waiting_time as f64 / tasks.len() as f64;
    let average_turnaround_time = total_turnaround_time as f64 / tasks.len() as f64;

    (average_waiting_time, average_turnaround_time)
}

// Main entry point of the program
fn main() {
    let tasks = vec![
        Task { pid: 1, time_arrival: 0, burst_time: 10, remaining_time: 10, start_time: None, finish_time: None },
        Task { pid: 2, time_arrival: 5, burst_time: 3, remaining_time: 3, start_time: None, finish_time: None },
        Task { pid: 3, time_arrival: 8, burst_time: 6, remaining_time: 6, start_time: None, finish_time: None },
    ];

    // Testing FCFS scheduling
    let completed_fcfs = fcfs(tasks.clone());
    let (fcfs_avg_wait, fcfs_avg_turn) = calculate_metrics(&completed_fcfs);
    println!("FCFS Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", fcfs_avg_wait, fcfs_avg_turn);

    //TO DO:
    // SRTF scheduling 
    // Round Robin scheduling
}
