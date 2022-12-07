import sys 
import re 
import matplotlib.pyplot as plt 

file_name = sys.argv[1]
save_file = sys.argv[2]

f = open(file_name, 'r')
datalist = f.readlines()

x = []
y = []

it = iter(datalist)
for i, j in zip(it, it):
    size = int(re.search(r'\d+', i).group())
    time = 1/2 * float(re.search(r'\d+\.\d+', j).group())
    time_unit = 1 
    if 'm' in j : 
        time_unit = 0.001
    elif 'µ' in j:
        time_unit = 0.001 * 0.001
    
    time = time * time_unit 
    mb_per_sec = size / time * 0.001 * 0.001 # for G 
    x.append(size)
    y.append(mb_per_sec)

plt.xlabel("byte size")
plt.ylabel("GB/sec")
plt.plot(x, y)
plt.grid()
plt.savefig(save_file)
