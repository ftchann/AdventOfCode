import re, functools

def tokenize(expr):
    return [int(token) if '0' <= token[0] <= '9' else token
            for token in re.findall("\[|\]|\d+", expr)]


def explode(tokens):
    depth = 0
    for i in range(len(tokens) - 3):
        if depth >= 4 and tokens[i] == '[' and tokens[i + 3] == ']':

            for l in range(i - 1, 0, -1):
                if isinstance(tokens[l], int):
                    tokens[l] += tokens[i + 1]
                    break
            for r in range(i + 4, len(tokens)):
                if isinstance(tokens[r], int):
                    tokens[r] += tokens[i + 2]
                    break
            tokens[i:i + 4] = [0]
            return True
        if tokens[i] == '[':
            depth += 1
        if tokens[i] == ']':
            depth -= 1
    return False


def split(tokens):
    for i in range(len(tokens)):
        if isinstance(tokens[i], int) and tokens[i] >= 10:
            tokens[i:i + 1] = ['[',
                               tokens[i] // 2,
                               (tokens[i] + 1) // 2,
                               ']']
            return True
    return False


def reduct(tokens):
    while explode(tokens) or split(tokens):
        pass
    return tokens


def magnitude(tokens):
    if len(tokens) == 1:
        return tokens[0]
    for i in range(len(tokens) - 3):
        if tokens[i] == '[' and tokens[i + 3] == ']':
            tokens[i:i + 4] = [tokens[i + 1] * 3 + tokens[i + 2] * 2]
            return magnitude(tokens)


numbers = [tokenize(line.strip()) for line in open('input18.txt').readlines()]
total = functools.reduce(
    lambda left, right: reduct(['['] + left + right + [']']),
    numbers)
print("Part 1:", magnitude(total))


maxmag = 0
for left in numbers:
    for right in numbers:
        curr = reduct(['['] + left + right + [']'])
        maxmag = max(maxmag, magnitude(curr))
print("Part 2:", maxmag)