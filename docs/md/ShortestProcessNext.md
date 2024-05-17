verify the SPN (Shortest Process Next) scheduling algorithm output for the input.

The tasks are processed in the order of their burst times once they have arrived.

the step-by-step execution:

### Input Tasks
```
1 0 4
2 1 5
3 2 2
4 3 1
5 4 3
6 5 4
```

### Expected Output

1. **At time 0:** Task 1 arrives and starts since it's the only task.
   - `Time 0: Task 1 starts`
   - `Time 4: Task 1 finishes`

2. **At time 4:** Task 4 arrives. Among the tasks that have arrived, Task 4 has the shortest burst time.
   - `Time 4: Task 4 starts`
   - `Time 5: Task 4 finishes`

3. **At time 5:** Task 3 and Task 5 have arrived. Among them, Task 3 has the shortest burst time.
   - `Time 5: Task 3 starts`
   - `Time 7: Task 3 finishes`

4. **At time 7:** Task 5 and Task 6 have arrived. Among them, Task 5 has the shortest burst time.
   - `Time 7: Task 5 starts`
   - `Time 10: Task 5 finishes`

5. **At time 10:** Task 6 is the only task available.
   - `Time 10: Task 6 starts`
   - `Time 14: Task 6 finishes`

6. **At time 14:** Task 2 is the only task left.
   - `Time 14: Task 2 starts`
   - `Time 19: Task 2 finishes`

This aligns with the output you received:

```
SPN (Shortest Process Next) Output:
Time 0: Task 1 starts
Time 4: Task 1 finishes
Time 4: Task 4 starts
Time 5: Task 4 finishes
Time 5: Task 3 starts
Time 7: Task 3 finishes
Time 7: Task 5 starts
Time 10: Task 5 finishes
Time 10: Task 6 starts
Time 14: Task 6 finishes
Time 14: Task 2 starts
Time 19: Task 2 finishes
SPN Average Waiting Time: 4.17, Average Turnaround Time: 7.33
```

### Average Waiting Time and Turnaround Time Calculations

- **Waiting Time:** 
  - Task 1: 0
  - Task 2: 13
  - Task 3: 3
  - Task 4: 1
  - Task 5: 3
  - Task 6: 5
  - Average Waiting Time: (0 + 13 + 3 + 1 + 3 + 5) / 6 = 25 / 6 ≈ 4.17

- **Turnaround Time:** 
  - Task 1: 4
  - Task 2: 18
  - Task 3: 5
  - Task 4: 2
  - Task 5: 6
  - Task 6: 9
  - Average Turnaround Time: (4 + 18 + 5 + 2 + 6 + 9) / 6 = 44 / 6 ≈ 7.33

The values for average waiting time and turnaround time are also correct.

Your output is consistent with the expected behavior of the SPN scheduling algorithm. The function is now correctly implemented.