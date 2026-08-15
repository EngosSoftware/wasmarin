# Input data
x <- c(1,2,5,10,20,50,100,200,300,400,500,600,700,800,900,1000,2000,3000,4000,5000,6000,7000,8000,9000,10000,20000,30000,40000,50000,60000,70000,80000,90000,100000,200000,300000,400000,500000,600000,700000,800000,900000,1000000,2000000,3000000,4000000,5000000,6000000,7000000,8000000,9000000,10000000)
y <- c(116750,118650,117890,119650,128900,131380,134380,137850,140310,143310,144690,146480,149320,149500,151140,152700,157770,161380,160700,161090,165040,164190,166910,170510,169440,177830,180690,176580,184910,184950,178980,208080,188830,210530,223160,222060,167750,174650,197210,223940,204280,191710,219310,359050,390860,366720,1822300,2256400,2506000,2882400,3333900,3781700)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating linear function
a <- 0.375
b <- 120000.0
fl <- function(x) {
  a*x + b
}

# Estimated linear values
xl <- seq(1, 10000000, by = 1000)
yl <- fl(xl)

# Estimating staircase function
base <- 120000
size <- 1024
cost <- 384
fs <- function(x) {
  base + (((x + size - 1) / size) * cost)
}

# Estimated staircase values
xs <- seq(1, 10000000, by = 1000)
ys <- fs(xs)

png("elem-drop-reg.png")
plot(x, y, main = "elem.drop reg Linux")
abline(model, col = "blue")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("bottomright", legend = sprintf("y = %.0f + %.5f x", coefs[1], coefs[2]), bty = "n")

png("elem-drop-log.png")
plot(x, y, log = "x", main = "elem.drop log linux")
lines(xl, yl, col = "magenta")
lines(xs, ys, col = "green", lty = 2)
legend("topleft", legend = sprintf(" base = %i\n size = %i\n cost = %i", base, size, cost), bty = "n")
