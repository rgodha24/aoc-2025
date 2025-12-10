import re
import sys

from z3 import Int, Optimize, Sum, sat


def parse_line(line: str):
    line = line.strip()
    rb = line.index("]")

    lcurly = line.index("{", rb)
    rcurly = line.index("}", lcurly)
    targets_str = line[lcurly + 1 : rcurly]
    targets = [int(x.strip()) for x in targets_str.split(",") if x.strip()]

    middle = line[rb + 1 : lcurly]
    button_strs = re.findall(r"\(([^)]*)\)", middle)
    buttons = []
    for bs in button_strs:
        bs = bs.strip()
        idxs = [int(x.strip()) for x in bs.split(",") if x.strip()]
        buttons.append(idxs)

    return (buttons, targets)


def min_presses_for_machine(buttons, targets):
    num_buttons = len(buttons)
    num_counters = len(targets)

    opt = Optimize()
    x = [Int(f"x_{i}") for i in range(num_buttons)]

    for xi in x:
        opt.add(xi >= 0)

    for c in range(num_counters):
        contributing = [x[j] for j, btn in enumerate(buttons) if c in btn]
        opt.add(Sum(contributing) == targets[c])

    total_presses = Sum(x)
    opt.minimize(total_presses)

    if opt.check() != sat:
        raise ValueError("No solution for this machine")
    model = opt.model()
    return model.eval(total_presses).as_long()


lines = [line for line in sys.stdin.read().splitlines() if line.strip()]
print(sum(map(lambda l: min_presses_for_machine(*parse_line(l)), lines)))
