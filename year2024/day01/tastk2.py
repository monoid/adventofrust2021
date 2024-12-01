import sys
from collections import defaultdict

def main():
    arr1 = []
    arr2 = defaultdict(int)
    for line in sys.stdin:
        (s1, s2) = line.split()
        n1 = int(s1)
        n2 = int(s2)
        arr1.append(n1)
        arr2[n2] += 1
    sum = 0
    for v1 in arr1:
        sum += v1 * arr2.get(v1, 0)
    print(sum)


if __name__ == '__main__':
    main()
