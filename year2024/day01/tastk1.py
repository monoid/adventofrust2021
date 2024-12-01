import sys

def main():
    arr1 = []
    arr2 = []
    for line in sys.stdin:
        (s1, s2) = line.split()
        n1 = int(s1)
        n2 = int(s2)
        arr1.append(n1)
        arr2.append(n2)
    arr1.sort()
    arr2.sort()
    sum = 0
    for (v1, v2) in zip(arr1, arr2):
        sum += abs(v1 - v2)
    print(sum)


if __name__ == '__main__':
    main()
