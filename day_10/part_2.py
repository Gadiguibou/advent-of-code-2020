from time import perf_counter_ns
from functools import cache

# naive solution -> O(N^3)
def naive(joltages, current_index):
    current = joltages[current_index]
    if current == 0:
        return 1
    
    minus_one_index = None
    minus_two_index = None
    minus_three_index = None
    
    next = current_index - 1
    
    if joltages[next] == current - 1:
        minus_one_index = next
        next -= 1
    if joltages[next] == current - 2:
        minus_two_index = next
        next -= 1
    if joltages[next] == current - 3:
        minus_three_index = next
    
    minus_one_result = naive(joltages, minus_one_index) if minus_one_index is not None else 0
    minus_two_result = naive(joltages, minus_two_index) if minus_two_index is not None else 0
    minus_three_result = naive(joltages, minus_three_index) if minus_three_index is not None else 0

    return minus_one_result + minus_two_result + minus_three_result


def main():
    f = open("./input")
    joltages = [int(l) for l in f]
    joltages.append(0)
    joltages.sort()
    joltages.append(joltages[-1] + 3)

    # still naive but better ;) -> O(N)
    # include function here to reuse global variable "joltages" and including the array in the dict key
    @cache
    def not_as_naive(current_index):
        current = joltages[current_index]
        if current == 0:
            return 1

        minus_one_index = None
        minus_two_index = None
        minus_three_index = None

        next = current_index - 1

        if joltages[next] == current - 1:
            minus_one_index = next
            next -= 1
        if joltages[next] == current - 2:
            minus_two_index = next
            next -= 1
        if joltages[next] == current - 3:
            minus_three_index = next

        minus_one_result = not_as_naive(minus_one_index) if minus_one_index is not None else 0
        minus_two_result = not_as_naive(minus_two_index) if minus_two_index is not None else 0
        minus_three_result = not_as_naive(minus_three_index) if minus_three_index is not None else 0

        return minus_one_result + minus_two_result + minus_three_result

    start_t = perf_counter_ns()
    print(not_as_naive(len(joltages) - 1))
    # print(naive(joltages, len(joltages) - 1))
    end_t = perf_counter_ns()
    print(end_t - start_t)


if __name__ == "__main__":
    main()