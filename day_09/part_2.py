def main():
    f = open("./test_input")
    previous_output = 127

    numbers = [int(l) for l in f.readlines()]
    

    for i in range(len(numbers)):
        contiguous_sum = numbers[i]
        next_index = i + 1
        while contiguous_sum <= previous_output:
            if contiguous_sum == previous_output:
                print(calculate_result(numbers[i:next_index]))
                return
            contiguous_sum += numbers[next_index]
            next_index += 1

    print("Reached end of list")
    return


def calculate_result(num_list):
    return max(num_list) + min(num_list)

if __name__ == "__main__":
    main()