
## Priority Scheduling Verification

This document verifies the Priority Scheduling output for the given input tasks.

### Input Tasks
```
1 0 4 3
2 1 5 1
3 2 2 4
4 3 1 2
5 4 3 5
6 5 4 6
```

### Priority Scheduling

The step-by-step execution of the Priority Scheduling is as follows:

1. **Time 0:**
   - Task 1 arrives and starts.
   - `Time 0: Task 1 arrived and added to the queue`
   - `Time 0: Task 1 starts`

2. **Time 4:**
   - Task 1 finishes.
   - Task 2, Task 3, Task 4, and Task 5 arrive and are added to the queue.
   - Task 2 starts.
   - `Time 4: Task 1 finishes`
   - `Time 4: Task 2 arrived and added to the queue`
   - `Time 4: Task 3 arrived and added to the queue`
   - `Time 4: Task 4 arrived and added to the queue`
   - `Time 4: Task 5 arrived and added to the queue`
   - `Time 4: Task 2 starts`

3. **Time 9:**
   - Task 2 finishes.
   - Task 6 arrives and is added to the queue.
   - Task 4 starts.
   - `Time 9: Task 2 finishes`
   - `Time 9: Task 6 arrived and added to the queue`
   - `Time 9: Task 4 starts`

4. **Time 10:**
   - Task 4 finishes.
   - Task 3 starts.
   - `Time 10: Task 4 finishes`
   - `Time 10: Task 3 starts`

5. **Time 12:**
   - Task 3 finishes.
   - Task 5 starts.
   - `Time 12: Task 3 finishes`
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
- Task 3: 10 - 2 = 8
- Task 4: 9 - 3 = 6
- Task 5: 12 - 4 = 8
- Task 6: 15 - 5 = 10

Average Waiting Time: (0 + 3 + 8 + 6 + 8 + 10) / 6 ≈ 5.83

#### Turnaround Time

- Task 1: 4 - 0 = 4
- Task 2: 9 - 1 = 8
- Task 3: 12 - 2 = 10
- Task 4: 10 - 3 = 7
- Task 5: 15 - 4 = 11
- Task 6: 19 - 5 = 14

Average Turnaround Time: (4 + 8 + 10 + 7 + 11 + 14) / 6 ≈ 9.00

### Conclusion

The Priority Scheduling output is correct and aligns with the expected behavior of the algorithm:

```
Priority Scheduling Output:
Time 0: Task 1 arrived and added to the queue
Time 0: Task 1 starts
Time 4: Task 1 finishes
Time 4: Task 2 arrived and added to the queue
Time 4: Task 3 arrived and added to the queue
Time 4: Task 4 arrived and added to the queue
Time 4: Task 5 arrived and added to the queue
Time 4: Task 2 starts
Time 9: Task 2 finishes
Time 9: Task 6 arrived and added to the queue
Time 9: Task 4 starts
Time 10: Task 4 finishes
Time 10: Task 3 starts
Time 12: Task 3 finishes
Time 12: Task 5 starts
Time 15: Task 5 finishes
Time 15: Task 6 starts
Time 19: Task 6 finishes
Priority Average Waiting Time: 5.83, Average Turnaround Time: 9.00
```

### External Verification

The output is also verified with the website: [https://process-scheduling-solver.boonsuen.com/](https://process-scheduling-solver.boonsuen.com/)

#### Screenshot of Website Verification
![Test Image](/docs/img/priority-verify.png)

This output confirms the correctness of the Priority Scheduling algorithm implementation and its verification using an external tool.