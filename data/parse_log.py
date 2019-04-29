import subprocess
import sys
from os import listdir
from os.path import isfile, join
import matplotlib.pyplot as plt
import re


def sorted_nicely(l):
    """ Sort the given iterable in the way that humans expect."""
    def convert(text): return int(text) if text.isdigit() else text

    def alphanum_key(key): return [convert(c)
                                   for c in re.split('([0-9]+)', key)]
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
            if len(words) == 1 and words[0].endswith("µs"):
                c_time = int(words[0].rstrip("µs"))
                if c_time > worst_time:
                    worst_time = c_time
                time += int(words[0].rstrip("µs"))
            if words[0] == "Time":
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
static = []
static_wtime = []
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
                        file], timeout=10.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_test",
                        file], timeout=10.0)
    except subprocess.TimeoutExpired:
        pass

    # We run the audio for 60s using the TimeOutExpired exception
    try:
        subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_test",
                        file], timeout=10.0)
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

    # Parse the log for static scheduling execution
    atime, wtime = parse_file("tmp/static_sched_log.txt")
    static.append(atime)
    static_wtime.append(wtime)

# print(x)
# print(seq)
# print(static)
# print(dynamic)

plt.plot(x, seq, label='Sequenciel Temps Moyen')
plt.plot(x, dynamic, label='Work Stealing')
plt.plot(x, static, label='Static Scheduling')
plt.legend()

plt.title('average cycle time for :'+sys.argv[1])
plt.ylabel('time (µs)')
plt.xlabel('number of nodes')

# plt.show()
plt.savefig('tmp/average.png',bbox_inches='tight')

plt.close()

plt.plot(x, seq_wtime, label='Pire Temps Sequenciel')
plt.plot(x, dynamic_wtime, label='Pire Temps Work Stealing')
plt.plot(x, static_wtime, label='Pire Temps Static')
plt.legend()

plt.title('worst cycle time for : '+sys.argv[1])
plt.ylabel('time (µs)')
plt.xlabel('number of nodes')

# plt.show()
plt.savefig('tmp/worst.png',bbox_inches='tight')
plt.close()
