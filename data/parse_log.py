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
        number = 0

        for line in file:
            words = line.strip().split(" ")
            if len(words) == 1 and words[0].endswith("µs") and number > 15:
                current_time = int(words[0].rstrip("µs"))
                if current_time > worst_time:
                    worst_time = current_time
                time += int(words[0].rstrip("µs"))
            if words[0] == "Time" and number > 15:
                next += int(words[5].rstrip("µs"))
            number += 1

        average_time = time / number
        average_next = next / number

        print("\nResults for " + path + ":")
        print("Cycles count: " + str(number))
        print("Worst time: " + str(worst_time) + "µs")
        print("Average time: " + str(average_time) + "µs")
        print("Average time left before the deadline: "
              + str(average_next) + "µs")

    return (average_time, worst_time)


dags = [f for f in listdir(sys.argv[1]) if isfile(join(sys.argv[1], f))]
dags = sorted_nicely(dags)

x = []
seq = []
seq_wtime = []
static_rand = []
static_rand_wtime = []
static_hlfet = []
static_hlfet_wtime = []
static_etf = []
static_etf_wtime = []
dynamic = []
dynamic_wtime = []


for dag in dags:

    file = sys.argv[1] + "/" + dag

    print("**********************************")
    print("File : " + file)

    nodes = ""
    for char in dag:
        if char.isdigit():
            nodes = nodes + char

    x.append(nodes)

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "seq_test",
                        file], timeout=1.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                        file, "rand"], timeout=1.0)
    except subprocess.TimeoutExpired:
        pass

    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                        file, "hlfet"], timeout=1.0)
    except subprocess.TimeoutExpired:
        pass

    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                        file, "etf"], timeout=1.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_test",
                        file], timeout=1.0)
    except subprocess.TimeoutExpired:
        pass

    # Parse the log for sequential execution
    atime, wtime = parse_file("tmp/seq_log.txt")
    seq.append(atime)
    seq_wtime.append(wtime)

    # Parse the log for work stealing execution
    atime, wtime = parse_file("tmp/work_stealing_log.txt")
    dynamic.append(atime)
    dynamic_wtime.append(wtime)

    # Parse the log for rand static scheduling execution
    atime, wtime = parse_file("tmp/static_rand_sched_log.txt")
    static_rand.append(atime)
    static_rand_wtime.append(wtime)

    # Parse the log for hlfet static scheduling execution
    atime, wtime = parse_file("tmp/static_hlfet_sched_log.txt")
    static_hlfet.append(atime)
    static_hlfet_wtime.append(wtime)

    # Parse the log for etf static scheduling execution
    atime, wtime = parse_file("tmp/static_etf_sched_log.txt")
    static_etf.append(atime)
    static_etf_wtime.append(wtime)


plt.plot(x, seq , label='Sequenciel Temps Moyen')
plt.plot(x, dynamic, label='Work Stealing')
plt.plot(x, static_rand , label='Static Rand Scheduling')
plt.plot(x, static_hlfet , label='Static HLFET Scheduling')
plt.plot(x, static_etf , label='Static ETF Scheduling')

plt.legend()

plt.title('average cycle time for :'+sys.argv[1])
plt.ylabel('time (µs)')
plt.xlabel('number of nodes')

# plt.show()
plt.savefig('tmp/average.png', bbox_inches='tight')
plt.close()

plt.plot(x, seq_wtime, label='Pire Temps Sequenciel')
plt.plot(x, dynamic_wtime, label='Pire Temps Work Stealing')
plt.plot(x, static_rand_wtime, label='Pire Temps Static Rand')
plt.plot(x, static_hlfet_wtime, label='Pire Temps Static HLFET')
plt.plot(x, static_etf_wtime, label='Pire Temps Static')
plt.legend()

plt.title('worst cycle time for : '+sys.argv[1])
plt.ylabel('time (µs)')
plt.xlabel('number of nodes')

# plt.show()
plt.savefig('tmp/worst.png', bbox_inches='tight')
plt.close()
