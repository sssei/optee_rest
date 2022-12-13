import sys 
import re 
import matplotlib.pyplot as plt 

file_name = sys.argv[1]

f = open(file_name, 'r')
datalist = f.readlines()

x = []
y1 = []
y2 = []

it = iter(datalist)
for i, j in zip(it, it):
    size = int(re.search(r'\d+', i).group())
    time = 1/2 * float(re.search(r'\d+', j).group())
    time_unit = 1 
    if 'm' in j : 
        time_unit = 1000
    elif 'µ' in j:
        time_unit = 1000 * 1000
    elif 'n' in j:
        time_unit = 1000 * 1000 * 1000
    
    Gbyte_unit = 0.001 * 0.001
    mb_per_sec = size / time * Gbyte_unit * time_unit
    msec = time / time_unit * 1000 
    x.append(size)
    y1.append(mb_per_sec)
    y2.append(msec)

fig = plt.figure(1)
plt.subplot(1, 1, 1)
plt.title("MB/sec")
plt.plot(x, y1)
plt.xlabel("byte size")
plt.ylabel("MB/sec")
plt.grid()
fig.savefig(file_name + ".png")

fig = plt.figure(2)
plt.subplot(1, 1, 1)
plt.title("RTT")
plt.plot(x, y2)
plt.xlabel("byte size")
plt.ylabel("msec")
plt.grid()
fig.savefig(file_name + "_RTT.png")


