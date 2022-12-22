import re 

tee_file = "2022-12-21_23-17_file_TEE"

data_list = []

for i in range(1,11):
    f = open(tee_file + str(i), 'r')
    data_list.append(f.readlines())

post_list = []
get_list = []
delete_list = []

for data in data_list: 
    it1 = iter(data)
    post_time = []
    get_time = []
    delete_time = []
    for s, l1, l2, l3 in zip(it1, it1, it1, it1):
        size = int(re.search(r'\d+', s).group())
        post_time.append(int(re.search(r'\d+', l1).group()))
        get_time.append(int(re.search(r'\d+', l2).group()))
        delete_time.append(int(re.search(r'\d+', l3).group()))
    post_list.append(post_time)
    get_list.append(get_time)
    delete_list.append(delete_time)

f = open("collected_file_TEE", 'w')

for i in range(0, 61):
    size = 256 * (i + 1)
    post = 0
    get = 0
    delete = 0
    for j in range(0, 10):
        post += post_list[j][i]
        get += get_list[j][i]
        delete += delete_list[j][i]
    f.write(str(size) + "\n")
    f.write(str(post) + "\n") 
    f.write(str(get) + "\n") 
    f.write(str(delete) + "\n")         



