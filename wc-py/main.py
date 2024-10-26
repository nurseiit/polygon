import sys

OPTIONS = ["-c", "-l", "-w", "-m"]


def word_count(options: list[str], file_content: bytes) -> list[int]:
    output = []

    if "-l" in options or not options:
        lines_count = len(file_content.splitlines())
        output.append(lines_count)
    if "-w" in options or not options:
        words_count = len(file_content.split())
        output.append(words_count)
    if "-c" in options or not options:
        bytes_count = len(file_content)
        output.append(bytes_count)
    if "-m" in options:
        chars_len = len(file_content.encode("utf-8"))
        output.append(chars_len)

    return output


def result_to_str(output: list[int]) -> str:
    return " ".join(list(map(lambda x: str(x), output)))


def main():
    args = sys.argv[1:]

    options = list(filter(lambda x: x.startswith("-"), args))
    invalid_options = list(filter(lambda x: x not in OPTIONS, options))

    if invalid_options:
        print("invalid option(s): {}".format(", ".join(invalid_options)))
        exit(1)

    file_names = list(filter(lambda x: x not in OPTIONS, args))

    if not file_names:
        file_content = sys.stdin.read().encode("utf-8")
        print(result_to_str(word_count(options, file_content)))
        exit(0)

    all_results: list[list[int]] = []

    for file_name in file_names:
        with open(file_name, "rb") as f:
            file_content = f.read()
        result = word_count(options, file_content)
        all_results.append(result)
        print("{} {}".format(result_to_str(result), file_name))

    if len(all_results) > 1:
        result = [sum(col) for col in zip(*all_results)]
        print("{} {}".format(result_to_str(result), "total"))


if __name__ == "__main__":
    main()
