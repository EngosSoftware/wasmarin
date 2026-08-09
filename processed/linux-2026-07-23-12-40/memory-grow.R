# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,65535)
y <- c(2307700,2293200,2297600,2288700,2267300,2282600,2281100,2289900,2289700,2277300,2299200,2282500,2289500,2286400,2299000,2321200)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating function
fe <- function(x) {
  2300000 + (((x + 8192 - 1) / 8192) * 32)
}

# Estimated values
xe <- seq(1, 65535, by = 1000)
ye <- fe(xe)

png("memory-grow-reg.png")
plot(x, y, main = "memory.grow reg Linux")
abline(model, col = "blue")
lines(xe, ye, col = "magenta")
legend("bottomright", legend = sprintf("y = %.0f + %.5f x", coefs[1], coefs[2]), bty = "n")

png("memory-grow-log.png")
plot(x, y, log = "x", main = "memory.grow log Linux")
lines(xe, ye, col = "magenta")
