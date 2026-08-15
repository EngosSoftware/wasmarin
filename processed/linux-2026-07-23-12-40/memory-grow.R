# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,65535)
y <- c(2307700,2293200,2297600,2288700,2267300,2282600,2281100,2289900,2289700,2277300,2299200,2282500,2289500,2286400,2299000,2321200)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating linear function
a <- 0.003906
b <- 2300000.0
fl <- function(x) {
  a*x + b
}

# Estimated linear values
xl <- seq(1, 10000000, by = 10000)
yl <- fl(xl)

# Estimating staircase function
base <- 2300000
size <- 8192
cost <- 32
fs <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated staircase values
xs <- seq(1, 65535, by = 1000)
ys <- fs(xs)

png("memory-grow-reg.png")
plot(x, y, main = "memory.grow reg Linux")
abline(model, col = "blue")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("bottomright", legend = sprintf("y = %.0f + %.5f x", coefs[1], coefs[2]), bty = "n")

png("memory-grow-log.png")
plot(x, y, log = "x", main = "memory.grow log Linux")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
