from sys import argv
import matplotlib.pyplot as plt

with open(argv[1], "r") as f:
    data = [int(line.strip()) for line in f if line.strip()]

# 画直方图
plt.hist(data, bins=50)

plt.title("Histogram")
plt.xlabel("Value")
plt.ylabel("Frequency")

plt.show()
