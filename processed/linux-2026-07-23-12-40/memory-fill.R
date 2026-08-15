# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,100000,200000,500000,1000000,2000000,5000000,10000000,20000000,50000000,100000000,200000000,500000000,1000000000,2000000000,4000000000)
y <- c(2845900,2844900,2850300,2849300,2847000,2842200,2849300,2849500,2848000,2867300,2882500,4807100,6680300,10370000,24901000,46441000,89888000,223290000,442920000,885460000,2503700000,5012200000,9964900000,24784000000,49592000000,99197000000,248310000000,500350000000,1004400000000,2011400000000)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating linear function
a <- 512.0
b <- 2900000.0
fl <- function(x) {
  a*x + b
}

# Estimated linear values
xl <- seq(1, 4000000000, by = 10000)
yl <- fl(xl)

# Estimating staircase function
base <- 2900000
size <- 64
cost <- 32768
fs <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated staircase values
xs <- seq(1, 4000000000, by = 10000)
ys <- fs(xs)

png("memory-fill-reg.png")
plot(x, y, main = "memory.fill reg Linux")
abline(model, col = "blue")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("bottomright", legend = sprintf("y = %.0f + %.2f x", coefs[1], coefs[2]), bty = "n")

png("memory-fill-log.png")
plot(x, y, log = "x", main = "memory.fill log Linux")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
