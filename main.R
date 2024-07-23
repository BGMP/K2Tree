# Load necessary libraries
library(ggplot2)
library(readr)

# Read the CSV file
data <- read_csv("data/time.csv")

# Create the plot
ggplot(data) +
  geom_point(aes(x = n, y = bitmatrix, color = "BitMatrix"), size = 2) +
  geom_point(aes(x = n, y = k2tree, color = "K2Tree"), size = 2) +
  geom_smooth(aes(x = n, y = bitmatrix, color = "BitMatrix"), method = "loess") +
  geom_smooth(aes(x = n, y = k2tree, color = "K2Tree"), method = "loess") +
  scale_x_log10() +
  scale_y_log10() +
  labs(title = "Log-Scaled Query Time",
       x = "Matrix Size (n)",
       y = "Time (nanoseconds)",
       color = "Legend") +
  theme_minimal() +
  scale_color_manual(values = c("BitMatrix" = "green", "K2Tree" = "orange"))
