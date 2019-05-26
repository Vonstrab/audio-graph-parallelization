import math
import subprocess
import sys
from os import listdir
from os.path import isfile, join
import matplotlib.pyplot as plt
import re


def convert(text):
    return int(text) if text.isdigit() else text


def alphanum_key(key):
    return [convert(c) for c in re.split('([0-9]+)', key)]


def sorted_nicely(l):
    """
    Sort the given iterable in the way that humans expect
    """
    return sorted(l, key=alphanum_key)


cycle_time = (2 * int(sys.argv[3]) / 44100)
print(cycle_time)

cycle_time = cycle_time * 1000000
print(cycle_time)


def parse_file(path):
    """
    Function for parsing a file containing measurements
    """
    with open(path, "r") as file:
        worst_time = 0
        time = 0
        next = 0
        nb_times = 0
        nb_next = 0
        misses = 0

        for line in file:
            words = line.strip().split(" ")
            if len(words) == 2 and words[1].endswith("µs"):
                current_time = int(words[1].rstrip("µs"))
                if current_time > worst_time:
                    worst_time = current_time
                if current_time > cycle_time:
                    misses += 1
                time += current_time
                nb_times += 1
            if words[0] == "Time":
                next += int(words[5].rstrip("µs"))
                nb_next += 1

        average_time = time // nb_times
        average_next = next // nb_next

        print("\nResults for " + path + ":")
        print("Cycles count: " + str(nb_times))
        print("Worst time: " + str(worst_time) + "µs")
        print("Average time: " + str(average_time) + "µs")
        print("Average time left before the deadline: "
              + str(average_next) + "µs")

    return (average_time, worst_time, misses)


if len(sys.argv) != 4:
    print("Usage: parse_log.py <AG Directory> <Number of threads> <buffer size>")
    sys.exit(-1)

dags = [f for f in listdir(sys.argv[1]) if isfile(join(sys.argv[1], f))]
dags = sorted_nicely(dags)

nb_threads = sys.argv[2]

x = []
seq = []
seq_wtime = []
seq_misses = []
# static_rand = []
# static_rand_wtime = []
static_hlfet = []
static_hlfet_wtime = []
static_hlfet_misses = []
static_etf = []
static_etf_wtime = []
static_etf_misses = []
dynamic = []
dynamic_wtime = []
dynamic_misses = []


subprocess.run(["cargo", "build", "--release", "--bin", "seq_exec"])
subprocess.run(["cargo", "build", "--release", "--bin", "static_sched_exec"])
subprocess.run(["cargo", "build", "--release", "--bin", "work_stealing_exec"])

for dag in dags:

    file = sys.argv[1].rstrip("/ ") + "/" + dag

    print("**********************************")
    print("File : " + file)

    nodes = ""
    for char in dag:
        if char.isdigit():
            nodes = nodes + char

    x.append(nodes)

    # We run the audio for 2s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "seq_exec",
                        file], timeout=3.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 2s using the TimeOutExpired exception
    # try:
    #     subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
    #                     file, nb_threads, "rand"], timeout=5.0)
    # except subprocess.TimeoutExpired:
    #     pass

    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                        file, nb_threads, "hlfet"], timeout=3.0)
    except subprocess.TimeoutExpired:
        pass

    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                        file, nb_threads, "etf"], timeout=3.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 2s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_exec",
                        file, nb_threads], timeout=3.0)
    except subprocess.TimeoutExpired:
        pass

    # Parse the log for sequential execution
    atime, wtime, misses = parse_file("tmp/seq_log.txt")
    seq.append(atime)
    seq_wtime.append(wtime)
    seq_misses.append(misses)

    # Parse the log for work stealing execution
    atime, wtime, misses = parse_file("tmp/work_stealing_log.txt")
    dynamic.append(atime)
    dynamic_wtime.append(wtime)
    dynamic_misses.append(misses)

    # # Parse the log for rand static scheduling execution
    # atime, wtime ,misses= parse_file(
    #     "tmp/static_rand_sched_log.txt")
    # static_rand.append(atime)
    # static_rand_wtime.append(wtime)

    # Parse the log for hlfet static scheduling execution
    atime, wtime, misses = parse_file(
        "tmp/static_hlfet_sched_log.txt")
    static_hlfet.append(atime)
    static_hlfet_wtime.append(wtime)
    static_hlfet_misses.append(misses)

    # Parse the log for etf static scheduling execution
    atime, wtime, misses = parse_file("tmp/static_etf_sched_log.txt")
    static_etf.append(atime)

    static_etf_wtime.append(wtime)
    static_etf_misses.append(misses)


plt.plot(x, seq, 'r+', label='Sequential Scheduling')
plt.plot(x, dynamic, 'b^', label='Work Stealing Scheduling')
# plt.plot(x, static_rand, 'gx', label='Static Rand Scheduling')
plt.plot(x, static_hlfet, 'bx', label='Static HLFET Scheduling')
plt.plot(x, static_etf, 'rx', label='Static ETF Scheduling')
plt.legend()


plt.title('Average execution time:'+sys.argv[1])
plt.ylabel('Time (µs)')
plt.xlabel('Number of nodes')

plt.savefig('tmp/average.png', bbox_inches='tight')
plt.close()

plt.plot(x, seq_wtime, 'r+', label='Sequential Scheduling')
plt.plot(x, dynamic_wtime, 'b^', label='Work Stealing Scheduling')
# plt.plot(x, static_rand_wtime, 'gx', label='Static Rand Scheduling')
plt.plot(x, static_hlfet_wtime, 'bx', label='Static HLFET Scheduling')
plt.plot(x, static_etf_wtime, 'rx', label='Static ETF Scheduling')
plt.legend()

plt.title('Worst execution time:' + sys.argv[1])
plt.ylabel('Time (µs)')
plt.xlabel('Number of nodes')

plt.savefig('tmp/worst.png', bbox_inches='tight')
plt.close()

plt.plot(x, seq_misses, 'r+', label='Sequential Scheduling')
plt.plot(x, dynamic_misses, 'b^', label='Work Stealing Scheduling')
# plt.plot(x, static_rand_wtime, 'gx', label='Static Rand Scheduling')
plt.plot(x, static_hlfet_misses, 'bx', label='Static HLFET Scheduling')
plt.plot(x, static_etf_misses, 'rx', label='Static ETF Scheduling')
plt.legend()

plt.title('Deadlines Missed:' + sys.argv[1])
plt.ylabel('Count (µs)')
plt.xlabel('Number of nodes')

plt.savefig('tmp/misses.png', bbox_inches='tight')
plt.close()
