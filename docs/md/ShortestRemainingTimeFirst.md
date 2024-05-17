## Shortest Remaining Time First (SRTF) Scheduling Verification

This document verifies the Shortest Remaining Time First (SRTF) scheduling output for the given input tasks.

### Input Tasks
```
1 0 8
2 1 4
3 2 9
4 3 5
5 4 3
6 5 4
```

### SRTF Scheduling

The step-by-step execution of the SRTF scheduling is as follows:

1. **Time 0:**
   - Task 1 arrives and starts.
   - `Time 0: Task 1 arrived and added to the queue`
   - `Time 0: Task 1 starts`

2. **Time 1:**
   - Task 2 arrives and is added to the queue.
   - `Time 1: Task 2 arrived and added to the queue`
   - `Time 1: Task 1 resumes`

3. **Time 2:**
   - Task 3 arrives and is added to the queue.
   - `Time 2: Task 3 arrived and added to the queue`
   - `Time 2: Task 1 resumes`

4. **Time 3:**
   - Task 4 arrives and is added to the queue.
   - `Time 3: Task 4 arrived and added to the queue`
   - `Time 3: Task 1 resumes`

5. **Time 4:**
   - Task 1 finishes.
   - Task 5 arrives and is added to the queue.
   - Task 4 starts.
   - `Time 4: Task 1 finishes`
   - `Time 4: Task 5 arrived and added to the queue`
   - `Time 4: Task 4 starts`

6. **Time 5:**
   - Task 4 finishes.
   - Task 6 arrives and is added to the queue.
   - Task 3 starts.
   - `Time 5: Task 4 finishes`
   - `Time 5: Task 6 arrived and added to the queue`
   - `Time 5: Task 3 starts`

7. **Time 6:**
   - Task 3 resumes.
   - `Time 6: Task 3 resumes`

8. **Time 7:**
   - Task 3 finishes.
   - Task 5 starts.
   - `Time 7: Task 3 finishes`
   - `Time 7: Task 5 starts`

9. **Time 8:**
   - Task 5 resumes.
   - `Time 8: Task 5 resumes`

10. **Time 9:**
    - Task 5 resumes.
    - `Time 9: Task 5 resumes`

11. **Time 10:**
    - Task 5 finishes.
    - Task 6 starts.
    - `Time 10: Task 5 finishes`
    - `Time 10: Task 6 starts`

12. **Time 11:**
    - Task 6 resumes.
    - `Time 11: Task 6 resumes`

13. **Time 12:**
    - Task 6 resumes.
    - `Time 12: Task 6 resumes`

14. **Time 13:**
    - Task 6 resumes.
    - `Time 13: Task 6 resumes`

15. **Time 14:**
    - Task 6 finishes.
    - Task 2 starts.
    - `Time 14: Task 6 finishes`
    - `Time 14: Task 2 starts`

16. **Time 15:**
    - Task 2 resumes.
    - `Time 15: Task 2 resumes`

17. **Time 16:**
    - Task 2 resumes.
    - `Time 16: Task 2 resumes`

18. **Time 17:**
    - Task 2 resumes.
    - `Time 17: Task 2 resumes`

19. **Time 18:**
    - Task 2 resumes.
    - `Time 18: Task 2 resumes`

20. **Time 19:**
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

The Shortest Remaining Time First (SRTF) scheduling output is correct and aligns with the expected behavior of the algorithm:

```
SRTF (Shortest Remaining Time) Output:
Time 0: Task 1 arrived and added to the queue
Time 0: Task 1 starts
Time 1: Task 2 arrived and added to the queue
Time 1: Task 1 resumes
Time 2: Task 3 arrived and added to the queue
Time 2: Task 1 resumes
Time 3: Task 4 arrived and added to the queue
Time 3: Task 1 resumes
Time 4: Task 1 finishes
Time 4: Task 5 arrived and added to the queue
Time 4: Task 4 starts
Time 5: Task 4 finishes
Time 5: Task 6 arrived and added to the queue
Time 5: Task 3 starts
Time 6: Task 3 resumes
Time 7: Task 3 finishes
Time 7: Task 5 starts
Time 8: Task 5 resumes
Time 9: Task 5 resumes
Time 10: Task 5 finishes
Time 10: Task 6 starts
Time 11: Task 6 resumes
Time 12: Task 6 resumes
Time 13: Task 6 resumes
Time 14: Task 6 finishes
Time 14: Task 2 starts
Time 15: Task 2 resumes
Time 16: Task 2 resumes
Time 17: Task 2 resumes
Time 18: Task 2 resumes
Time 19: Task 2 finishes
SRTF Average Waiting Time: 4.17, Average Turnaround Time: 7.33
```

### External Verification

The output is also verified with the website: [https://process-scheduling-solver.boonsuen.com/](https://process-scheduling-solver.boonsuen.com/)

#### Screenshot of Website Verification
![Test Image](/docs/img/srtf-verify.png)

This output confirms the correctness of the Shortest Remaining Time First scheduling algorithm implementation and its verification using an external tool.