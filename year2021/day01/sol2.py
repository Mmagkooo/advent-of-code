import sys

depths = [int(line) for line in sys.stdin]
cnt = 0

window_size = 3
window = sum(depths[:window_size])
for i in range(3, len(depths)):
    new_window = window + depths[i] - depths[i-3]
    if new_window > window:
        cnt += 1
    window = new_window

print(cnt)
