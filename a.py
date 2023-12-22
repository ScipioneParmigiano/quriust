import matplotlib.pyplot as plt
from collections import Counter

# Sample data (replace this with your data)
data = []
# Calculate frequencies of each element using Counter
frequency_count = Counter(data)

# Separate values and their corresponding frequencies
values = list(frequency_count.keys())
frequencies = list(frequency_count.values())

# Create a histogram
plt.bar(values, frequencies)

# Add labels and title
plt.xlabel('Values')
plt.ylabel('Frequency')
plt.title('Histogram of Frequencies')

# Show the plot
plt.show()
