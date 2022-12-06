import sys
import itertools

N_STEPS = int(sys.argv[1])
BUFFER_SIZE = 3 * N_STEPS

char2int = { ".": 0, "#": 1 }

ADJ = list(itertools.product(range(-1, 2), range(-1, 2)))

rules, _, *image_lines = [line.strip() for line in sys.stdin]
assert len(rules) == 512

def print_image(image: dict, default):
    min_pixel = min(image.keys())
    max_pixel = max(image.keys())

    for i in range(min_pixel[0], max_pixel[0] + 1):
        for j in range(min_pixel[1], max_pixel[1] + 1):
            pixel = i, j
            print(image.get(pixel, default), end="")
        print()
    print()

def enhance(image: dict, default):
    new_image = {}
    for i, j in image:
        new_val = 0
        for di, dj in ADJ:
            npixel = i + di, j + dj
            nchar = image.get(npixel, default)
            nint = 0 if nchar == "." else 1
            new_val = 2 * new_val + nint
        new_image[i, j] = rules[new_val]
    return new_image

desired_height = len(image_lines) + BUFFER_SIZE * 2
desired_width = len(image_lines[0]) + BUFFER_SIZE * 2

default = [".", "#"] # indexable
default_index = char2int["."]

image = {}
for i in range(desired_height):
    for j in range(desired_width):
        image[i, j] = default[default_index]

for i, image_line in enumerate(image_lines):
    for j, pixel in enumerate(image_line):
        image[BUFFER_SIZE + i, BUFFER_SIZE + j] = pixel

for _ in range(N_STEPS):
    image = enhance(image, default[default_index])
    default_index = 1 - default_index

def count(image: dict):
    cnt = 0
    for pixel in image: cnt += char2int[image[pixel]]
    return cnt

print(count(image))
