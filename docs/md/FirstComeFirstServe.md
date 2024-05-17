## First Come First Serve (FCFS) Scheduling Verification

This document verifies the First Come First Serve (FCFS) scheduling output for the given input tasks.

### Input Tasks
```
1 0 4
2 1 5
3 2 2
4 3 1
5 4 3
6 5 4
```

### FCFS Scheduling

The step-by-step execution of the FCFS scheduling is as follows:

1. **Time 0:**
   - Task 1 starts.
   - `Time 0: Task 1 starts`

2. **Time 4:**
   - Task 1 finishes.
   - Task 2 starts.
   - `Time 4: Task 1 finishes`
   - `Time 4: Task 2 starts`

3. **Time 9:**
   - Task 2 finishes.
   - Task 3 starts.
   - `Time 9: Task 2 finishes`
   - `Time 9: Task 3 starts`

4. **Time 11:**
   - Task 3 finishes.
   - Task 4 starts.
   - `Time 11: Task 3 finishes`
   - `Time 11: Task 4 starts`

5. **Time 12:**
   - Task 4 finishes.
   - Task 5 starts.
   - `Time 12: Task 4 finishes`
   - `Time 12: Task 5 starts`

6. **Time 15:**
   - Task 5 finishes.
   - Task 6 starts.
   - `Time 15: Task 5 finishes`
   - `Time 15: Task 6 starts`

7. **Time 19:**
   - Task 6 finishes.
   - `Time 19: Task 6 finishes`

### Average Waiting Time and Turnaround Time Calculations

#### Waiting Time

- Task 1: 0 - 0 = 0
- Task 2: 4 - 1 = 3
- Task 3: 9 - 2 = 7
- Task 4: 11 - 3 = 8
- Task 5: 12 - 4 = 8
- Task 6: 15 - 5 = 10

Average Waiting Time: (0 + 3 + 7 + 8 + 8 + 10) / 6 = 36 / 6 = 6.00

#### Turnaround Time

- Task 1: 4 - 0 = 4
- Task 2: 9 - 1 = 8
- Task 3: 11 - 2 = 9
- Task 4: 12 - 3 = 9
- Task 5: 15 - 4 = 11
- Task 6: 19 - 5 = 14

Average Turnaround Time: (4 + 8 + 9 + 9 + 11 + 14) / 6 = 55 / 6 ≈ 9.17

### Conclusion

The First Come First Serve (FCFS) scheduling output is correct and aligns with the expected behavior of the algorithm:

```
FCFS (First Come First Serve) Output:
Time 0: Task 1 starts
Time 4: Task 1 finishes
Time 4: Task 2 starts
Time 9: Task 2 finishes
Time 9: Task 3 starts
Time 11: Task 3 finishes
Time 11: Task 4 starts
Time 12: Task 4 finishes
Time 12: Task 5 starts
Time 15: Task 5 finishes
Time 15: Task 6 starts
Time 19: Task 6 finishes
FCFS Average Waiting Time: 6.00, Average Turnaround Time: 9.17
```

### External Verification

The output is also verified with the website: [https://process-scheduling-solver.boonsuen.com/](https://process-scheduling-solver.boonsuen.com/)

#### Screenshot of Website Verification
![Test Image](/docs/img/fcfs-verify.png)

This output confirms the correctness of the First Come First Serve scheduling algorithm implementation and its verification using an external tool.