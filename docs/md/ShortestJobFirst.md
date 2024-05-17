## Shortest Job Next (SJN) Scheduling Verification

This document verifies the Shortest Job Next (SJN) scheduling output for the given input tasks.

### Input Tasks
```
1 0 4
2 1 5
3 2 2
4 3 1
5 4 3
6 5 4
```

### SJN Scheduling

The step-by-step execution of the SJN scheduling is as follows:

1. **Time 0:**
   - Task 1 starts.
   - `Time 0: Task 1 starts`

2. **Time 4:**
   - Task 1 finishes.
   - Task 4 starts.
   - `Time 4: Task 1 finishes`
   - `Time 4: Task 4 starts`

3. **Time 5:**
   - Task 4 finishes.
   - Task 3 starts.
   - `Time 5: Task 4 finishes`
   - `Time 5: Task 3 starts`

4. **Time 7:**
   - Task 3 finishes.
   - Task 5 starts.
   - `Time 7: Task 3 finishes`
   - `Time 7: Task 5 starts`

5. **Time 10:**
   - Task 5 finishes.
   - Task 6 starts.
   - `Time 10: Task 5 finishes`
   - `Time 10: Task 6 starts`

6. **Time 14:**
   - Task 6 finishes.
   - Task 2 starts.
   - `Time 14: Task 6 finishes`
   - `Time 14: Task 2 starts`

7. **Time 19:**
   - Task 2 finishes.
   - `Time 19: Task 2 finishes`

### Average Waiting Time and Turnaround Time Calculations

#### Waiting Time

- Task 1: 0 - 0 = 0
- Task 2: 14 - 1 = 13
- Task 3: 5 - 2 = 3
- Task 4: 4 - 3 = 1
- Task 5: 7 - 4 = 3
- Task 6: 10 - 5 = 5

Average Waiting Time: (0 + 13 + 3 + 1 + 3 + 5) / 6 = 25 / 6 ≈ 4.17

#### Turnaround Time

- Task 1: 4 - 0 = 4
- Task 2: 19 - 1 = 18
- Task 3: 7 - 2 = 5
- Task 4: 5 - 3 = 2
- Task 5: 10 - 4 = 6
- Task 6: 14 - 5 = 9

Average Turnaround Time: (4 + 18 + 5 + 2 + 6 + 9) / 6 = 44 / 6 ≈ 7.33

### Conclusion

The Shortest Job Next (SJN) scheduling output is correct and aligns with the expected behavior of the algorithm:

```
SJN (Shortest Job Next) Output:
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
SJN Average Waiting Time: 4.17, Average Turnaround Time: 7.33
```

### External Verification

The output is also verified with the website: [https://process-scheduling-solver.boonsuen.com/](https://process-scheduling-solver.boonsuen.com/)

#### Screenshot of Website Verification
![Test Image](/docs/img/sjf-spn-verify.png)

This output confirms the correctness of the Shortest Job Next scheduling algorithm implementation and its verification using an external tool.