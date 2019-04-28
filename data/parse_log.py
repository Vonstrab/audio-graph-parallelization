import time
import os
import subprocess
import sys

#we run the audio for 60 s using the TimeOutExpired excepction
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "seq_test",
                    sys.argv[1]], timeout=60.0)
except subprocess.TimeoutExpired:
    pass

#we run the audio for 60 s using the TimeOutExpired excepction
try:
    subprocess.run(["cargo", "run", "--release", "--bin", "work_stealing_test",
                    sys.argv[1]], timeout=60.0)
except subprocess.TimeoutExpired:
    pass

#then we parse the seq log
fichier = open("tmp/seq_log.txt", "r")

temps = 0
next = 0
numero = 0

for line in fichier:
    tab = line.split(" ")
    if len(tab) > 1 and tab[1] == "µs\n":
        temps += int(tab[0])
    if tab[0] == "Temps":
        next += int(tab[6])
    numero += 1

temps_moy = temps / numero
next_moy = next / numero

print("execution seq")
print("nobre de cycles "+str(numero))
print("temps moyen "+str(temps_moy)+" µs")
print("temps moyen avant prochain cycle "+str(next_moy)+" µs")

#then we parse the work steal log
fichier = open("tmp/work_steal_log.txt", "r")

temps = 0
next = 0
numero = 0

for line in fichier:
    tab = line.split(" ")
    if len(tab) > 1 and tab[1] == "µs\n":
        temps += int(tab[0])
    if tab[0] == "Temps":
        next += int(tab[6])
    numero += 1

temps_moy = temps / numero
next_moy = next / numero

print("\nWork stealing")
print("nobre de cycles "+str(numero))
print("temps moyen "+str(temps_moy)+" µs")
print("temps moyen avant prochain cycle "+str(next_moy)+" µs")