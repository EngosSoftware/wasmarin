# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,100000,200000,500000,1000000,2000000,5000000,9999999)
y <- c(99013,99764,102730,106230,120070,117080,129200,156840,247650,421560,894890,2357600,5056900,10817000,27778000,57067000,126240000,330740000,946480000,2074000000,5485900000,11079000000)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating function
base <- 70000
size <- 32
cost <- 34816
fe <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated values
xe <- seq(1, 9999999, by = 1000)
ye <- fe(xe)

png("table-copy-reg.png")
plot(x, y, main = "table.copy reg Linux")
abline(model, col = "blue")
lines(xe, ye, col = "magenta")
legend("bottomright", legend = sprintf("y = %.0f + %.2f x", coefs[1], coefs[2]), bty = "n")

png("table-copy-log.png")
plot(x, y, log = "x", main = "table.copy log Linux")
lines(xe, ye, col = "magenta")
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
