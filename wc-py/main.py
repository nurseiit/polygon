import sys

OPTIONS = ["-c", "-l", "-w", "-m"]


def main():
    args = sys.argv[1:]

    options = list(filter(lambda x: x.startswith("-"), args))
    invalid_options = list(filter(lambda x: x not in OPTIONS, options))

    if invalid_options:
        print("invalid option(s): {}".format(", ".join(invalid_options)))
        exit(1)

    file_names = list(filter(lambda x: x not in OPTIONS, args))

    if not file_names:
        file_content = sys.stdin.read()
        bytes_count = len(file_content.encode("utf-8"))
    else:
        file_name = file_names[0]
        with open(file_name) as f:
            file_content = f.read()
            bytes_count = f.tell()

    output: list[int] = []

    if "-l" in options or not options:
        lines_count = len(file_content.splitlines())
        output.append(lines_count)

    if "-w" in options or not options:
        words_count = len(file_content.split())
        output.append(words_count)

    if "-c" in options or not options:
        output.append(bytes_count)

    if "-m" in options:
        chars_len = len(file_content.encode("utf-8"))
        output.append(chars_len)

    result = " ".join(list(map(lambda x: str(x), output)))

    if file_names:
        print("{} {}".format(result, file_name))
    else:
        print(result)


if __name__ == "__main__":
    main()
