import sys 
import re 
import datetime 
import matplotlib.pyplot as plt 

if len(sys.argv) != 4:
    print("Usage : python3 %s <tee_file> <normal_file> <title> " % sys.argv[0])
    sys.exit()

tee_file = sys.argv[1] 
normal_file = sys.argv[2]
title = sys.argv[3]

f1 = open(tee_file, 'r')
f2 = open(normal_file, 'r')
data_tee = f1.readlines()
data_normal = f2.readlines()

x = []
y_tee = []
y_normal = []

it1 = iter(data_tee)
for i, j in zip(it1, it1):
    size = int(re.search(r'\d+', i).group())
    time = float(re.search(r'\d+', j).group())
    time_unit = 1000 * 1000 * 1000
    msec = time / time_unit * 1000 
    x.append(size)
    y_tee.append(msec)

it2 = iter(data_normal)
n = 0
for i, j in zip(it2, it2):
    size = int(re.search(r'\d+', i).group())
    time = float(re.search(r'\d+', j).group())
    time_unit = 1000 * 1000 * 1000 
    msec = time / time_unit * 1000 
    if x[n] != size :
        print("size error")
        sys.exit()
    n += 1
    y_normal.append(msec)

dt_now = datetime.datetime.now()
file_name = dt_now.strftime("%Y-%m-%d_%H-%M")

fig = plt.figure()
plt.title(title)
plt.plot(x, y_tee, label='TEE')
plt.plot(x, y_normal, label='REE')
plt.xlabel("byte size")
plt.ylabel("RTT(ms)")
#plt.yscale("log")
plt.legend()
plt.grid()
fig.savefig(file_name + ".png")


