import matplotlib.pyplot as plt

arr = []

arr += [0 for o in range(0, 35)]

count = 0
for i in range(36, 70):
    count += 1
    arr.append(count * 1 / 35)

arr += [1 for o in range(70, 100)]

plt.plot(arr)
plt.show()