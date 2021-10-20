from collections import deque

def main():
    f = open("./input")
        
    buffer = deque()

    # for _ in range(5): # test input
    for _ in range(25): # main input
        next = int(f.readline())
        buffer.append(next)
    
    for line in f:
        next = int(line)
        if not pair_exists(next, buffer):
            print(next)
            return
        buffer.popleft()
        buffer.append(next)
    return

def pair_exists(n, buf):
    for first in buf:
        for second in buf:
            if first == second:
                continue
            if first + second == n:
                return True
    
    return False


if __name__ == "__main__":
    main()
