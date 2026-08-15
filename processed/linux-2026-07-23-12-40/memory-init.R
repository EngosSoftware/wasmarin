# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,100000,200000,500000,1000000,2000000,5000000,10000000,20000000,50000000,100000000,200000000,500000000,1000000000,2000000000,2147483647)
y <- c(3026500,3019500,3014700,3014200,3015700,3019000,3026600,3032700,3060200,3087500,3118700,5106600,7873400,11859000,28619000,54056000,104550000,256170000,512100000,1034900000,2597900000,5172300000,10261000000,25331000000,49592000000,99102000000,248620000000,500490000000,1005000000000,1079600000000)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating linear function
a <- 512.0
b <- 3100000.0
fl <- function(x) {
  a*x + b
}

# Estimated linear values
xl <- seq(1, 2147483647, by = 10000)
yl <- fl(xl)

# Estimating staircase function
base <- 3100000
size <- 64
cost <- 32768
fs <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated staircase values
xs <- seq(1, 2147483647, by = 10000)
ys <- fs(xs)

png("memory-init-reg.png")
plot(x, y, main = "memory.init reg Linux")
abline(model, col = "blue")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("bottomright", legend = sprintf("y = %.0f + %.2f x", coefs[1], coefs[2]), bty = "n")

png("memory-init-log.png")
plot(x, y, log = "x", main = "memory.init log Linux")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
