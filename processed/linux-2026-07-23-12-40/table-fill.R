# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,100000,200000,500000,1000000,2000000,5000000,9999999)
y <- c(100330,101660,104070,108470,115510,133870,170110,224540,380670,664690,1288100,3198300,6231800,12398000,30921000,61726000,123530000,315340000,688420000,1394600000,3541400000,7148300000)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating function
fe <- function(x) {
  80000 + (((x + 64 - 1) / 64) * 45056)
}

# Estimated values
xe <- seq(1, 40000000000, by = 10000)
ye <- fe(xe)

png("table-fill-reg.png")
plot(x, y, main = "table.fill reg Linux")
abline(model, col = "blue")
lines(xe, ye, col = "magenta")
legend("bottomright", legend = sprintf("y = %.0f + %.2f x", coefs[1], coefs[2]), bty = "n")

png("table-fill-log.png")
plot(x, y, log = "x", main = "table.fill log Linux")
lines(xe, ye, col = "magenta")
