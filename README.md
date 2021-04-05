# Gantt graph solver

Solves for a Gantt graph representation for CPU scheduling given a list of
processes and their arrival/service times.

## Example

### Given the inputs

 Process | Arrival time | Service time
---------|--------------|--------------
 A       | 0            | 3
 B       | 2            | 6
 C       | 4            | 4
 D       | 6            | 5
 E       | 8            | 2

### The program will return the following outputs

 t      | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
--------|---|---|---|---|---|---|---|---|---|---|----|----|----|----|----|----|----|----|----|----
 fifo   | A | A | A | B | B | B | B | B | B | C | C  | C  | C  | D  | D  | D  | D  | D  | E  | E
 rr q=4 | A | A | A | B | B | B | B | C | C | C | C  | D  | D  | D  | D  | B  | B  | E  | E  | D
 rr q=1 | A | A | B | A | B | C | B | D | C | B | E  | D  | C  | B  | E  | D  | C  | B  | D  | D
 sjf    | A | A | A | B | B | B | B | B | B | E | E  | C  | C  | C  | C  | D  | D  | D  | D  | D

        | Average turnaround time | Average response time
--------|-------------------------|----------------------
 fifo   | `8.6`                   | `4.6`
 rr q=4 | `10.0`                  | `3.6`
 rr q=1 | `10.8`                  | `0.8`
 sjf    | `7.6`                   | `3.6`
