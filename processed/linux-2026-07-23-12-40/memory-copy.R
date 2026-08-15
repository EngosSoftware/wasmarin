# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,100000,200000,500000,1000000,2000000,5000000,10000000,20000000,50000000,100000000,200000000,500000000,1000000000,2000000000,40000000000)
y <- c(4433500,4447900,4448700,4442500,4452100,4558100,4570700,4581700,4591600,4614000,4623800,7936800,11177000,17512000,43075000,80194000,156880000,386500000,765290000,1528700000,3840800000,7702000000,15617000000,39220000000,78225000000,155420000000,389660000000,784200000000,1565000000000,3133700000000)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating linear function
a <- 784
b <- 4500000.0
fl <- function(x) {
  a*x + b
}

# Estimated linear values
xl <- seq(1, 40000000000, by = 10000)
yl <- fl(xl)

# Estimating staircase function
base <- 4500000
size <- 64
cost <- 50176
fs <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated staircase values
xs <- seq(1, 40000000000, by = 10000)
ys <- fs(xs)

png("memory-copy-reg.png")
plot(x, y, main = "memory.copy reg Linux")
abline(model, col = "blue")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("bottomright", legend = sprintf("y = %.0f + %.2f x", coefs[1], coefs[2]), bty = "n")

png("memory-copy-log.png")
plot(x, y, log = "x", main = "memory.copy log Linux")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
