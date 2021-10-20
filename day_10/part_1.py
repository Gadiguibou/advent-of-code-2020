def main():
    f = open("./input")
    joltages = [int(l) for l in f]
    diff_counts = [0, 0, 0]

    joltages.sort()
    joltages.append(joltages[-1] + 3)
    previous_joltage = 0

    for joltage in joltages:
        diff = joltage - previous_joltage
        diff_counts[diff - 1] += 1
        previous_joltage = joltage

    print(diff_counts)
    print(diff_counts[0] * diff_counts[2])


if __name__ == "__main__":
    main()