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
        hist = []

        for line in file:
            words = line.strip().split(" ")
            if len(words) == 2 and words[1].endswith("µs"):
                current_time = int(words[1].rstrip("µs"))
                hist.append(current_time / 1000)
                if current_time > worst_time:
                    worst_time = current_time
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

    return (hist, average_time, worst_time)


if len(sys.argv) != 3:
    print("Usage: parse_log.py <AG Directory> <Number of threads>")
    sys.exit(-1)

dags = [f for f in listdir(sys.argv[1]) if isfile(join(sys.argv[1], f))]
dags = sorted_nicely(dags)

x = []
seq = []
seq_hist = []
seq_wtime = []
static_rand = []
static_rand_hist = []
static_rand_wtime = []
static_hlfet = []
static_hlfet_hist = []
static_hlfet_wtime = []
static_etf = []
static_etf_hist = []
static_etf_wtime = []
dynamic = []
dynamic_hist = []
dynamic_wtime = []

subprocess.run(["cargo", "build", "--release", "--bin", "seq_exec"])
subprocess.run(["cargo", "build", "--release", "--bin", "static_sched_exec"])
subprocess.run(["cargo", "build", "--release", "--bin", "work_stealing_exec"])

for dag in dags:

    file = sys.argv[1].rstrip("/ ") + "/" + dag
    nb_threads = sys.argv[2]

    print("**********************************")
    print("File : " + file)

    nodes = ""
    for char in dag:
        if char.isdigit():
            nodes = nodes + char

    x.append(nodes)

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "seq_exec",
                        file], timeout=2.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                        file, nb_threads, "rand"], timeout=2.0)
    except subprocess.TimeoutExpired:
        pass

    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                        file, nb_threads, "hlfet"], timeout=2.0)
    except subprocess.TimeoutExpired:
        pass

    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                        file, nb_threads, "etf"], timeout=2.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_exec",
                        file, nb_threads], timeout=2.0)
    except subprocess.TimeoutExpired:
        pass

    # Parse the log for sequential execution
    seq_hist, atime, wtime = parse_file("tmp/seq_log.txt")
    seq.append(atime)
    seq_wtime.append(wtime)

    # Parse the log for work stealing execution
    dynamic_hist, atime, wtime = parse_file("tmp/work_stealing_log.txt")
    dynamic.append(atime)
    dynamic_wtime.append(wtime)

    # Parse the log for rand static scheduling execution
    static_rand_hist, atime, wtime = parse_file(
        "tmp/static_rand_sched_log.txt")
    static_rand.append(atime)
    static_rand_wtime.append(wtime)

    # Parse the log for hlfet static scheduling execution
    static_hlfet_hist, atime, wtime = parse_file(
        "tmp/static_hlfet_sched_log.txt")
    static_hlfet.append(atime)
    static_hlfet_wtime.append(wtime)

    # Parse the log for etf static scheduling execution
    static_etf_hist, atime, wtime = parse_file("tmp/static_etf_sched_log.txt")
    static_etf.append(atime)
    static_etf_wtime.append(wtime)


plt.plot(x, seq, 'r+', label='Sequential Scheduling')
plt.plot(x, dynamic, 'b^', label='Work Stealing Scheduling')
plt.plot(x, static_rand, 'gx', label='Static Rand Scheduling')
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
plt.plot(x, static_rand_wtime, 'gx', label='Static Rand Scheduling')
plt.plot(x, static_hlfet_wtime, 'bx', label='Static HLFET Scheduling')
plt.plot(x, static_etf_wtime, 'rx', label='Static ETF Scheduling')
plt.legend()

plt.title('Worst execution time:' + sys.argv[1])
plt.ylabel('Time (µs)')
plt.xlabel('Number of nodes')

plt.savefig('tmp/worst.png', bbox_inches='tight')
plt.close()


plt.hist(seq_hist, bins=50, color='red')

plt.title('Sequential')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')

plt.savefig('tmp/hist_seq.png', bbox_inches='tight')
plt.close()


plt.hist(dynamic_hist, bins=50, color='green')

plt.title('Work Stealing')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')

plt.savefig('tmp/hist_ws.png', bbox_inches='tight')
plt.close()


plt.hist(static_rand_hist, bins=50, color='blue')

plt.title('Random static scheduling')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')

plt.savefig('tmp/hist_rand.png', bbox_inches='tight')
plt.close()


plt.hist(static_hlfet_hist, bins=50, color='grey')

plt.title('HLFET')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')

plt.savefig('tmp/hist_hlfet.png', bbox_inches='tight')
plt.close()


plt.hist(static_etf_hist, bins=50, color='black')

plt.title('ETF')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')

plt.savefig('tmp/hist_etf.png', bbox_inches='tight')
plt.close()
