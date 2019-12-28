from collections import Counter


def check_double(n):
    n = str(int(n))
    assert (len(n) == 6)
    nums = Counter([n[i:i + 2] for i in range(len(n) - 1) if n[i] == n[i + 1]])
    for i in nums.values():
        if i == 1:
            return True
    return False


def check_increase(n):
    n = str(int(n))
    for i in range(len(n) - 1):
        if n[i] > n[i + 1]:
            return False
    return True


def run_check(n):
    return True if check_double(n) and check_increase(n) else False


if __name__ == '__main__':
    i = 0
    for n in range(171309, 643603):
        i += 1 if run_check(n) else 0
    print(i)
