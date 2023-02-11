import sys 
import re 
import datetime 
import matplotlib.pyplot as plt 

tee_file = "../client/collected_file_TEE2"
normal_file = "../client/2023-02-09_14-37_file_REE"
title = "File I/O Time"

f1 = open(tee_file, 'r')
f2 = open(normal_file, 'r')
data_tee = f1.readlines()
data_normal = f2.readlines()

x = []
y1_tee = []
y2_tee = []
y3_tee = []
y1_normal = []
y2_normal = []
y3_normal = []

it1 = iter(data_tee)
for s, l1, l2, l3 in zip(it1, it1, it1, it1):
    size = int(re.search(r'\d+', s).group())
    post_time = float(re.search(r'\d+', l1).group()) / (1000 * 1000) 
    get_time = float(re.search(r'\d+', l2).group()) / (1000 * 1000)
    delete_time = float(re.search(r'\d+', l3).group()) / (1000 * 1000)
    x.append(size)
    y1_tee.append(post_time)
    y2_tee.append(get_time)    
    y3_tee.append(delete_time)        

it2 = iter(data_normal)
n = 0
for s, l1, l2, l3 in zip(it2, it2, it2, it2):
    size = int(re.search(r'\d+', s).group())
    post_time = float(re.search(r'\d+', l1).group()) / (1000 * 1000)
    get_time = float(re.search(r'\d+', l2).group()) / (1000 * 1000)    
    delete_time = float(re.search(r'\d+', l3).group()) / (1000 * 1000)    
    if x[n] != size :
        print("size error")
        sys.exit()
    n += 1
    y1_normal.append(post_time)
    y2_normal.append(get_time)
    y3_normal.append(delete_time)        

dt_now = datetime.datetime.now()
file_name = dt_now.strftime("%Y-%m-%d_%H-%M")

fig = plt.figure()
# plt.title(title)
plt.plot(x, y1_tee, label='Write(TEE)', color='tab:blue')
plt.plot(x, y2_tee, label='Read(TEE)', color='tab:orange')
plt.plot(x, y3_tee, label='Delete(TEE)', color='tab:green')
plt.plot(x, y1_normal, label='Write(REE)', linestyle='dashed', color='tab:blue')
plt.plot(x, y2_normal, label='Read(REE)', linestyle='dashed', color='tab:orange')
plt.plot(x, y3_normal, label='Delete(REE)', linestyle='dashed', color='tab:green')

plt.legend(bbox_to_anchor=(1,1))

plt.xlabel("byte size")
plt.ylabel("Elapsed Time (ms)")
# plt.yscale("log")
plt.ylim(bottom=0)
plt.grid()
fig.savefig(file_name + ".png", bbox_inches='tight')
