use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Task {
    pub pid: u32,  // Process ID
    pub time_arrival: u32,  // Arrival time of the task
    pub burst_time: u32,  // CPU burst time required by the task
    pub remaining_time: u32,  // Time remaining for the task completion (used in SRTF and RR)
    pub start_time: Option<u32>,  // Start time of the task in CPU
    pub finish_time: Option<u32>,  // Finish time of the task from CPU
}

pub fn fcfs(mut tasks: Vec<Task>) -> Vec<Task> {
    println!("FCFS (First Come First Serve) Output:");
    tasks.sort_by_key(|task| task.time_arrival);
    let mut current_time = 0;
    for task in tasks.iter_mut() {
        if current_time < task.time_arrival {
            println!("Time {}: Idle", current_time);
            current_time = task.time_arrival;
        }
        task.start_time = Some(current_time);
        println!("Time {}: Task {} starts", current_time, task.pid);
        current_time += task.burst_time;
        task.finish_time = Some(current_time);
        println!("Time {}: Task {} finishes", current_time, task.pid);
    }
    tasks
}

pub fn round_robin(mut tasks: Vec<Task>, quantum: u32) -> Vec<Task> {
    println!("RR (Round Robin) with time_quantum = {}", quantum);
    let mut queue = VecDeque::new();
    let mut current_time = 0;
    let mut completed_tasks = Vec::new();

    while !tasks.is_empty() || !queue.is_empty() {
        // Move tasks to the queue based on their arrival time
        while let Some(task) = tasks.first().cloned() {
            if task.time_arrival <= current_time {
                println!("Time {}: Task {} arrived and added to the queue", current_time, task.pid);
                queue.push_back(tasks.remove(0));
            } else {
                break;
            }
        }

        if let Some(mut task) = queue.pop_front() {
            if task.start_time.is_none() {
                task.start_time = Some(current_time);
                println!("Time {}: Task {} starts", current_time, task.pid);
            } else {
                println!("Time {}: Task {} resumes", current_time, task.pid);
            }

            let execution_time = std::cmp::min(task.remaining_time, quantum);
            current_time += execution_time;
            task.remaining_time -= execution_time;

            // Add newly arrived tasks to the queue during execution
            while let Some(task) = tasks.first().cloned() {
                if task.time_arrival <= current_time {
                    println!("Time {}: Task {} arrived and added to the queue", current_time, task.pid);
                    queue.push_back(tasks.remove(0));
                } else {
                    break;
                }
            }

            if task.remaining_time > 0 {
                println!("Time {}: Task {} preempted with {} time unit(s) remaining", current_time, task.pid, task.remaining_time);
                queue.push_back(task);
            } else {
                task.finish_time = Some(current_time);
                println!("Time {}: Task {} finishes", current_time, task.pid);
                completed_tasks.push(task);
            }
        } else {
            // Handle idle CPU time
            println!("Time {}: CPU Idle", current_time);
            if let Some(next_task) = tasks.first() {
                current_time = next_task.time_arrival;
            }
        }
    }

    completed_tasks
}

pub fn spn(mut tasks: Vec<Task>) -> Vec<Task> {
    let mut current_time = 0;
    let mut completed_tasks = Vec::new();

    while !tasks.is_empty() {
        // Filter tasks that have arrived
        let available_tasks: Vec<_> = tasks.iter().enumerate()
            .filter(|(_, task)| task.time_arrival <= current_time)
            .collect();

        if let Some((index, _)) = available_tasks.iter()
            .min_by_key(|(_, task)| task.burst_time) {
            let mut task = tasks.remove(*index);
            task.start_time = Some(current_time);
            println!("Time {}: Task {} starts", current_time, task.pid);
            current_time += task.burst_time;
            task.finish_time = Some(current_time);
            println!("Time {}: Task {} finishes", current_time, task.pid);
            completed_tasks.push(task);
        } else {
            println!("Time {}: CPU Idle", current_time);
            // Advance time to the next task's arrival if the CPU is idle
            if let Some(next_task) = tasks.iter().min_by_key(|task| task.time_arrival) {
                current_time = next_task.time_arrival;
            } else {
                break;
            }
        }
    }

    completed_tasks
}

pub fn sjf(tasks: Vec<Task>) -> Vec<Task> {
    println!("SJF (Shortest Job First) Output:");
    spn(tasks) // SJF is equivalent to SPN without preemption
}

// Function to calculate average waiting time and turnaround time
pub fn calculate_metrics(tasks: &[Task]) -> (f64, f64) {
    let total_waiting_time: u32 = tasks.iter().map(|task| task.finish_time.unwrap() - task.time_arrival - task.burst_time).sum();
    let total_turnaround_time: u32 = tasks.iter().map(|task| task.finish_time.unwrap() - task.time_arrival).sum();

    let average_waiting_time = total_waiting_time as f64 / tasks.len() as f64;
    let average_turnaround_time = total_turnaround_time as f64 / tasks.len() as f64;

    (average_waiting_time, average_turnaround_time)
}
