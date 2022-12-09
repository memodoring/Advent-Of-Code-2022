def read_file(filename: str) -> str:
    with open(filename, 'r') as file:
        return file.read().strip()


def day6() -> (int, int):
    file_name = 'puzzle.txt'
    file_data = read_file(file_name)

    def get_unique_char(data: str, start: int) -> int:
        read, count = data[:start], start
        for i in range(start, len(data)):
            if len(set(read[-start:])) == start:
                break

            read += data[i]
            count += 1

        return count

    def part1(data: str) -> (int, str):
        return get_unique_char(data, 4)

    def part2(data: str) -> int:
        return get_unique_char(data, 14)

    return part1(file_data), part2(file_data)


if __name__ == "__main__":
    print("Part 1: %d, Part 2: %d" % day6())