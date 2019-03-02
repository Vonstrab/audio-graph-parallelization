import networkx as nx
import matplotlib.pyplot as plt

import sys

import plotly.offline as py
import plotly.figure_factory as ff

df = []


f = open(sys.argv[1], "r")


for x in f:
    tab =x.split(" ")
    proc = tab[0]
    start= tab[1]
    end= tab[2]
    df.append (dict(Task=proc, Start=start, Finish=end))

fig = ff.create_gantt(df, group_tasks=True)
py.plot(fig, filename='gantt-chart')