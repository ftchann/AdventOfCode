from collections import deque, Counter
from itertools import combinations
import numpy as np
data = open("input19.txt").read()


def rotation_gen(scanner_list: np.ndarray):
    vectors = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    vectors = list(map(np.array, vectors))
    rots = []
    for vi in vectors:
        for vj in vectors:
            if vi.dot(vj) == 0:
                vk = np.cross(vi, vj)
                rot = np.array([vi, vj, vk])
                rots.append(rot)
    for r in rots:
        yield scanner_list.dot(r)


# Part 1
scanner_lines = [x.splitlines() for x in data.split('\n\n')]
scanners = [
    np.array([np.array(list(map(int, y.split(','))), dtype=int) for y in x[1:]])
    for x in scanner_lines
]

beacons = set(tuple(x) for x in scanners[0].astype(int))
remaining = deque(v for v in scanners[1:])
scanner_poss = list(np.array([0, 0, 0]))

while len(remaining):
    cur = remaining.pop()
    for rotd in rotation_gen(cur):
        c = Counter()
        for v in beacons:
            diff = rotd - np.array(v, dtype=int)
            l = [tuple(x) for x in diff]
            c.update(l)
        [(diff, count)] = c.most_common(1)
        if count >= 12:
            beacons |= set(tuple(x) for x in rotd - np.array(diff, dtype=int))
            scanner_poss.append(np.array(diff, dtype=int))
            break
    else:
        remaining.appendleft(cur)
print(f'Day 19 Part 1: {len(beacons)}')
# Part 2
d19p2 = max(np.abs(x - y).sum() for x, y in combinations(scanner_poss, 2))
print(f'Day 19 Part 2: {d19p2}')
