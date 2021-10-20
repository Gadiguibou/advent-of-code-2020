from time import perf_counter

# naive solution
def naive(joltages):


def main():
    f = open("./test_input")
    joltages = [int(l) for l in f]
    joltages.append(0)
    joltages.sort()
    joltages.append(joltages[-1] + 3)
    start_t = perf_counter()
    print(naive(joltages))
    end_t = perf_counter()
    print(end_t - start_t)


if __name__ == "__main__":
    main()