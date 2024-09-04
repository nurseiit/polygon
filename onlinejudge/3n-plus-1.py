from sys import stdin


cache = {}


def cycle_len(n):
    if n == 1:
        return 1

    if n in cache:
        return cache[n]

    if n % 2 == 1:
        cache[n] = 1 + cycle_len(3 * n + 1)
        return cache[n]

    cache[n] = 1 + cycle_len(n // 2)
    return cache[n]


def max_cycle(left, right):
    left, right = min(left, right), max(left, right)
    return max(cycle_len(i) for i in range(left, right + 1))


def main():
    for line in stdin:
        left, right = map(int, line.split()[:2])
        max_len = max_cycle(left, right)
        print(left, right, max_len)


if __name__ == "__main__":
    main()
