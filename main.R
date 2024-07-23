library(ggplot2)
library(scales)

data <- read.csv("data/time.csv")

n_breakpoints <- c(4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096)

time_plot <- ggplot(data) +
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
  scale_color_manual(values = c("BitMatrix" = "green", "K2Tree" = "orange")) +
  scale_x_log10(breaks = n_breakpoints)

ggsave("img/TIME.png", plot = time_plot, width = 10, height = 6, dpi = 300)

print(time_plot)

data <- read.csv("data/space.csv")

data$bitmatrix <- as.numeric(gsub("a", "", as.character(data$bitmatrix)))
data$k2tree <- as.numeric(gsub("a", "", as.character(data$k2tree)))
data <- na.omit(data)

space_plot <- ggplot(data) +
  geom_line(aes(x = n, y = bitmatrix, color = "BitMatrix"), size = 1) +
  geom_point(aes(x = n, y = bitmatrix, color = "BitMatrix"), size = 3) +
  geom_line(aes(x = n, y = k2tree, color = "K2Tree"), size = 1) +
  geom_point(aes(x = n, y = k2tree, color = "K2Tree"), size = 3) +
  scale_x_continuous(trans = 'log2') +
  scale_y_continuous(trans = 'log', labels = label_comma(accuracy = 1)) + # Format Y-axis labels without decimals
  labs(title = "BitMatrix and K2Tree Size vs. n",
       x = "n (Size of Matrix)",
       y = "Size (bytes)") +
  scale_color_manual(values = c("BitMatrix" = "green", "K2Tree" = "orange")) +
  theme_minimal() +
  theme(text = element_text(size = 12),
        axis.title = element_text(size = 14),
        axis.text = element_text(size = 10),
        plot.title = element_text(size = 16, face = "bold"))

print(space_plot)

ggsave("img/SPACE.png", plot = space_plot, width = 10, height = 6, dpi = 300)
