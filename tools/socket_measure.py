import sys 
import re 
import datetime 
import matplotlib.pyplot as plt 

tee_socket_file = "../client/2022-12-16_15-40_socket_TEE"
normal_socket_file = "../client/2022-12-16_15-49_socket_REE"
tee_tls_file = "../client/2022-12-13_18-00_tls_TEE"
normal_tls_file = "../client/2022-12-13_18-01_tls_REE"
title = "RTT"

f1 = open(tee_socket_file, 'r')
f2 = open(normal_socket_file, 'r')
f3 = open(tee_tls_file, 'r')
f4 = open(normal_tls_file, 'r')

socket_tee = f1.readlines()
socket_ree = f2.readlines()
tls_tee = f3.readlines()
tls_ree = f4.readlines()

x = []
y_socket_tee = []
y_socket_ree = []
y_tls_tee = []
y_tls_ree = []

it1 = iter(socket_tee)
for i, j in zip(it1, it1):
    size = int(re.search(r'\d+', i).group())
    time = float(re.search(r'\d+', j).group())
    time_unit = 1000 * 1000 * 1000
    msec = time / time_unit * 1000 
    x.append(size)
    y_socket_tee.append(msec)

it1 = iter(socket_ree)
for i, j in zip(it1, it1):
    size = int(re.search(r'\d+', i).group())
    time = float(re.search(r'\d+', j).group())
    time_unit = 1000 * 1000 * 1000
    msec = time / time_unit * 1000 
    y_socket_ree.append(msec)

it1 = iter(tls_tee)
for i, j in zip(it1, it1):
    size = int(re.search(r'\d+', i).group())
    time = float(re.search(r'\d+', j).group())
    time_unit = 1000 * 1000 * 1000
    msec = time / time_unit * 1000 
    y_tls_tee.append(msec)

it1 = iter(tls_ree)
for i, j in zip(it1, it1):
    size = int(re.search(r'\d+', i).group())
    time = float(re.search(r'\d+', j).group())
    time_unit = 1000 * 1000 * 1000
    msec = time / time_unit * 1000 
    y_tls_ree.append(msec)

dt_now = datetime.datetime.now()
file_name = dt_now.strftime("%Y-%m-%d_%H-%M")

fig = plt.figure()
# plt.title(title)
plt.plot(x, y_socket_tee, label='Socket TEE', color='tab:blue')
plt.plot(x, y_socket_ree, linestyle='dashed', label='Socket REE', color='tab:blue')
plt.plot(x, y_tls_tee, label='TLS TEE', color='tab:orange')
plt.plot(x, y_tls_ree, linestyle='dashed', label='TLS REE', color = 'tab:orange')
plt.xlabel("byte size")
plt.ylabel("RTT(ms)")
#plt.yscale("log")
plt.legend()
plt.ylim(bottom=0)
plt.grid()
fig.savefig(file_name + ".png")
