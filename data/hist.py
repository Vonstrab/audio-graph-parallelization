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

def get_scale(data):
    max_count = 0
    max_time = 0.0

    for d in data:
        for x in d:
            if x > max_time:
                max_time = x

    max_time = math.ceil(max_time) + 1

    for d in data:
        n, bins, patches = plt.hist(d, bins=100, range=(0, max_time))

        for count in n:
            if count > max_count:
                max_count = count

        plt.clf()

    return max_time, max_count + 1



def parse_file(path):
    """
    Function for parsing a file containing measurements
    """
    with open(path, "r") as file:
        hist = []

        for line in file:
            words = line.strip().split(" ")
            if len(words) == 2 and words[1].endswith("µs"):
                current_time = int(words[1].rstrip("µs"))
                if (current_time / 1000 < 100):
                    hist.append(current_time / 1000)

    return hist


if len(sys.argv) != 3:
    print("Usage: parse_log.py <AG File> <Number of threads>")
    sys.exit(-1)

dag = sys.argv[1]

seq_hist = []
static_rand_hist = []
static_hlfet_hist = []
static_etf_hist = []
dynamic_hist = []

subprocess.run(["cargo", "build", "--release", "--bin", "seq_exec"])
subprocess.run(["cargo", "build", "--release", "--bin", "static_sched_exec"])
subprocess.run(["cargo", "build", "--release", "--bin", "work_stealing_exec"])

nb_threads = sys.argv[2]

print("**********************************")
print("File : " + dag)

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "seq_exec",
                    dag], timeout=6.0)
except subprocess.TimeoutExpired:
    pass

# We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                    dag, nb_threads, "rand"], timeout=6.0)
except subprocess.TimeoutExpired:
    pass

try:
    subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                    dag, nb_threads, "hlfet"], timeout=6.0)
except subprocess.TimeoutExpired:
    pass

try:
    subprocess.run(["cargo", "run", "--release", "--bin", "static_sched_exec",
                    dag, nb_threads, "etf"], timeout=6.0)
except subprocess.TimeoutExpired:
    pass

    # We run the audio for 60s using the TimeOutExpired exception
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_exec",
                    dag, nb_threads], timeout=6.0)
except subprocess.TimeoutExpired:
    pass

# Parse the log for sequential execution
seq_hist = parse_file("tmp/seq_log.txt")

# Parse the log for work stealing execution
dynamic_hist = parse_file("tmp/work_stealing_log.txt")

# Parse the log for rand static scheduling execution
static_rand_hist = parse_file(
    "tmp/static_rand_sched_log.txt")

# Parse the log for hlfet static scheduling execution
static_hlfet_hist = parse_file(
    "tmp/static_hlfet_sched_log.txt")

# Parse the log for etf static scheduling execution
static_etf_hist = parse_file("tmp/static_etf_sched_log.txt")

data = [seq_hist, dynamic_hist, static_hlfet_hist, static_etf_hist]
color = ['red', 'green', 'blue', 'grey']
xaxes = ['Cycle Time (ms)', 'Cycle Time (ms)',
         'Cycle Time (ms)', 'Cycle Time (ms)']
yaxes = ['Count', 'Count', 'Count', 'Count']
titles = ['Sequential', 'Work Stealing', 'HLFET', 'ETF']

f, a = plt.subplots(2, 2, sharex=True, sharey=True)
a = a.ravel()

for idx, ax in enumerate(a):

    ax.hist(data[idx], bins=100, color=color[idx])
    ax.set_title(titles[idx])
    ax.set_xlabel(xaxes[idx])
    ax.set_ylabel(yaxes[idx])
plt.tight_layout()

plt.savefig('tmp/hist_all.png')
plt.close()

max_x, max_y = get_scale(
    [seq_hist, dynamic_hist, static_rand_hist, static_hlfet_hist, static_etf_hist])


plt.hist(seq_hist, bins=100, range=(0, max_x), color='red')

plt.title('Sequential')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')
plt.xlim(0, max_x)
plt.ylim(0, max_y)

plt.savefig('tmp/hist_seq.png', bbox_inches='tight')
plt.close()


plt.hist(dynamic_hist, bins=100, range=(0, max_x), color='green')

plt.title('Work Stealing')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')
plt.xlim(0, max_x)
plt.ylim(0, max_y)

plt.savefig('tmp/hist_ws.png', bbox_inches='tight')
plt.close()


plt.hist(static_rand_hist, bins=100, range=(0, max_x), color='blue')

plt.title('Random static scheduling')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')
plt.xlim(0, max_x)
plt.ylim(0, max_y)

plt.savefig('tmp/hist_rand.png', bbox_inches='tight')
plt.close()


plt.hist(static_hlfet_hist, bins=100, range=(0, max_x), color='grey')

plt.title('HLFET')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')
plt.xlim(0, max_x)
plt.ylim(0, max_y)

plt.savefig('tmp/hist_hlfet.png', bbox_inches='tight')
plt.close()


plt.hist(static_etf_hist, bins=100, range=(0, max_x), color='black')

plt.title('ETF')
plt.xlabel('Cycle Time (ms)')
plt.ylabel('Count')
plt.xlim(0, max_x)
plt.ylim(0, max_y)

plt.savefig('tmp/hist_etf.png', bbox_inches='tight')
plt.close()
