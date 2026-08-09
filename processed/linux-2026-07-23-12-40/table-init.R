# Input data
x <- c(1,2,5,10,20,50,100,200,500,1000,2000,5000,10000,20000,50000,100000,200000,500000,1000000,2000000,5000000,9999999)
y <- c(109980,111030,116860,123170,135740,175100,251600,378330,769230,1414600,2715000,6589400,13064000,25967000,64378000,129120000,258340000,736940000,1542400000,3173700000,8080900000,16240000000)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating function
base <- 70000
size <- 32
cost <- 52224
fe <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated values
xe <- seq(1, 9999999, by = 1000)
ye <- fe(xe)

png("table-init-reg.png")
plot(x, y, main = "table.init reg Linux")
abline(model, col = "blue")
lines(xe, ye, col = "magenta")
legend("bottomright", legend = sprintf("y = %.0f + %.2f x", coefs[1], coefs[2]), bty = "n")

png("table-init-log.png")
plot(x, y, log = "x", main = "table.init log Linux")
lines(xe, ye, col = "magenta")
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
