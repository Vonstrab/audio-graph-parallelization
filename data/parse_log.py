import subprocess
import sys

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "seq_test",
                    sys.argv[1]], timeout=60.0)
except subprocess.TimeoutExpired:
    pass

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_test",
                    sys.argv[1]], timeout=60.0)
except subprocess.TimeoutExpired:
    pass

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                    sys.argv[1], "rand"], timeout=60.0)
except subprocess.TimeoutExpired:
    pass

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                    sys.argv[1], "hlfet"], timeout=60.0)
except subprocess.TimeoutExpired:
    pass

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                    sys.argv[1], "etf"], timeout=60.0)
except subprocess.TimeoutExpired:
    pass


def parse_file(path):
    """
    Function for parsing a file containing measurements
    """
    with open(path, "r") as file:
        time = 0
        next = 0
        number = 0

        for line in file:
            words = line.strip().split(" ")
            if len(words) == 1 and words[0].endswith("µs"):
                time += int(words[0].rstrip("µs"))
            if words[0] == "Time":
                next += int(words[5].rstrip("µs"))
            number += 1

        average_time = time / number
        average_next = next / number

        print("\nResults for " + path + ":")
        print("Cycles count: " + str(number))
        print("Average time: " + str(average_time) + "µs")
        print("Average time left before the deadline: "
              + str(average_next) + "µs")


# Parse the log for sequential execution
parse_file("tmp/seq_log.txt")

# Parse the log for work stealing execution
parse_file("tmp/work_stealing_log.txt")

# Parse the logs for static scheduling execution
parse_file("tmp/static_rand_sched_log.txt")
parse_file("tmp/static_hlfet_sched_log.txt")
parse_file("tmp/static_etf_sched_log.txt")
