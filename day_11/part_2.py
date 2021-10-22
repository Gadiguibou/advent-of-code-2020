def main():
    f = open("./input")
    current_seats = [[c for c in l.strip()] for l in f]

    while True:
        seats_next_turn = [
            [state_next_round(current_seats, x, y) for y, _ in enumerate(row)]
            for x, row in enumerate(current_seats)
        ]

        if seats_next_turn == current_seats:
            break
        else:
            current_seats = seats_next_turn
            for row in seats_next_turn:
                print("".join(row))
            print()
    
    final_occupied_seat_count = sum(
        sum(1 if seat == "#" else 0 for seat in row)
        for row in current_seats
    )

    print(final_occupied_seat_count)


def state_next_round(seats, x0, y0):
    if seats[x0][y0] == ".":
        return "."

    occupied_adjacent_seats = 0

    for delta_x, delta_y in [(-1, -1), (-1, 0), (-1, 1),
                 ( 0, -1),          ( 0, 1),
                 ( 1, -1), ( 1, 0), ( 1, 1)]:

        x = x0 + delta_x
        y = y0 + delta_y

        while 0 <= x < len(seats) and 0 <= y < len(seats[delta_x]):
            match seats[x][y]:
                case "L":
                    break
                case "#":
                    occupied_adjacent_seats += 1
                    break
                case ".":
                    x += delta_x
                    y += delta_y
                    continue
                case _:
                    raise ValueError(f"Seat cannot be character \"{seats[x][y]}\"")
        
    if occupied_adjacent_seats == 0:
        return '#'
    elif occupied_adjacent_seats < 5:
        return seats[x0][y0]
    else:
        return 'L'


if __name__ == "__main__":
    main()