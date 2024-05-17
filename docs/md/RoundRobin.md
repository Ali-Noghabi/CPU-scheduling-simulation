## Round Robin (RR) Scheduling Verification

This document verifies the Round Robin (RR) scheduling output for the given input tasks and a time quantum of 2.

### Input Tasks
```
1 0 4
2 1 5
3 2 2
4 3 1
5 4 3
6 5 4
```

### Round Robin with Quantum = 2

The step-by-step execution of the Round Robin scheduling is as follows:

1. **Time 0:**
   - Task 1 arrives and starts.
   - `Time 0: Task 1 arrived and added to the queue`
   - `Time 0: Task 1 starts`

2. **Time 2:**
   - Task 1 is preempted with 2 time units remaining.
   - Task 2 and Task 3 arrive.
   - `Time 2: Task 1 preempted with 2 time unit(s) remaining`
   - `Time 2: Task 2 arrived and added to the queue`
   - `Time 2: Task 3 arrived and added to the queue`
   - Task 2 starts.
   - `Time 2: Task 2 starts`

3. **Time 4:**
   - Task 2 is preempted with 3 time units remaining.
   - Task 4 and Task 5 arrive.
   - `Time 4: Task 2 preempted with 3 time unit(s) remaining`
   - `Time 4: Task 4 arrived and added to the queue`
   - `Time 4: Task 5 arrived and added to the queue`
   - Task 3 starts.
   - `Time 4: Task 3 starts`

4. **Time 6:**
   - Task 3 finishes.
   - Task 1 resumes.
   - Task 6 arrives.
   - `Time 6: Task 3 finishes`
   - `Time 6: Task 1 resumes`
   - `Time 6: Task 6 arrived and added to the queue`

5. **Time 8:**
   - Task 1 finishes.
   - Task 4 starts.
   - `Time 8: Task 1 finishes`
   - `Time 8: Task 4 starts`

6. **Time 9:**
   - Task 4 finishes.
   - Task 5 starts.
   - `Time 9: Task 4 finishes`
   - `Time 9: Task 5 starts`

7. **Time 11:**
   - Task 5 is preempted with 1 time unit remaining.
   - Task 2 resumes.
   - `Time 11: Task 5 preempted with 1 time unit(s) remaining`
   - `Time 11: Task 2 resumes`

8. **Time 13:**
   - Task 2 is preempted with 1 time unit remaining.
   - Task 6 starts.
   - `Time 13: Task 2 preempted with 1 time unit(s) remaining`
   - `Time 13: Task 6 starts`

9. **Time 15:**
   - Task 6 is preempted with 2 time units remaining.
   - Task 5 resumes.
   - `Time 15: Task 6 preempted with 2 time unit(s) remaining`
   - `Time 15: Task 5 resumes`

10. **Time 16:**
    - Task 5 finishes.
    - Task 2 resumes.
    - `Time 16: Task 5 finishes`
    - `Time 16: Task 2 resumes`

11. **Time 17:**
    - Task 2 finishes.
    - Task 6 resumes.
    - `Time 17: Task 2 finishes`
    - `Time 17: Task 6 resumes`

12. **Time 19:**
    - Task 6 finishes.
    - `Time 19: Task 6 finishes`

### Average Waiting Time and Turnaround Time Calculations

#### Waiting Time

- Task 1: 8 - 4 = 4
- Task 2: 17 - 5 - 1 = 11
- Task 3: 6 - 2 - 2 = 2
- Task 4: 9 - 1 - 3 = 5
- Task 5: 16 - 3 - 4 = 9
- Task 6: 19 - 4 - 5 = 10

Average Waiting Time: (4 + 11 + 2 + 5 + 9 + 10) / 6 ≈ 6.83

#### Turnaround Time

- Task 1: 8 - 0 = 8
- Task 2: 17 - 1 = 16
- Task 3: 6 - 2 = 4
- Task 4: 9 - 3 = 6
- Task 5: 16 - 4 = 12
- Task 6: 19 - 5 = 14

Average Turnaround Time: (8 + 16 + 4 + 6 + 12 + 14) / 6 = 60 / 6 = 10.00

### Conclusion

The Round Robin (RR) scheduling output is correct and aligns with the expected behavior of the algorithm with a time quantum of 2:

```
RR (Round Robin) with time_quantum = 2
Time 0: Task 1 arrived and added to the queue
Time 0: Task 1 starts
Time 2: Task 1 preempted with 2 time unit(s) remaining
Time 2: Task 2 arrived and added to the queue
Time 2: Task 3 arrived and added to the queue
Time 2: Task 1 preempted with 2 time unit(s) remaining
Time 2: Task 2 starts
Time 4: Task 4 arrived and added to the queue
Time 4: Task 5 arrived and added to the queue
Time 4: Task 2 preempted with 3 time unit(s) remaining
Time 4: Task 3 starts
Time 6: Task 6 arrived and added to the queue
Time 6: Task 3 finishes
Time 6: Task 1 resumes
Time 8: Task 1 finishes
Time 8: Task 4 starts
Time 9: Task 4 finishes
Time 9: Task 5 starts
Time 11: Task 5 preempted with 1 time unit(s) remaining
Time 11: Task 2 resumes
Time 13: Task 2 preempted with 1 time unit(s) remaining
Time 13: Task 6 starts
Time 15: Task 6 preempted with 2 time unit(s) remaining
Time 15: Task 5 resumes
Time 16: Task 5 finishes
Time 16: Task 2 resumes
Time 17: Task 2 finishes
Time 17: Task 6 resumes
Time 19: Task 6 finishes

RR Average Waiting Time: 6.83, Average Turnaround Time: 10.00
```

### External Verification

The output is also verified with the website: [https://process-scheduling-solver.boonsuen.com/](https://process-scheduling-solver.boonsuen.com/)

#### Screenshot of Website Verification
![Test Image](/docs/img/rr-verify.png)

This output confirms the correctness of the Round Robin scheduling algorithm implementation. and its verification using an external tool.
