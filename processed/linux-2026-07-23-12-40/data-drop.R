# Input data
x <- c(1,2,5,10,20,50,100,200,300,400,500,600,700,800,900,1000,2000,3000,4000,5000,6000,7000,8000,9000,10000,20000,30000,40000,50000,60000,70000,80000,90000,100000,200000,300000,400000,500000,600000,700000,800000,900000,1000000,2000000,3000000,4000000,5000000,6000000,7000000,8000000,9000000,10000000)
y <- c(136370,136660,138160,137930,138450,139710,140050,149920,150070,150500,152890,152480,152310,152500,153020,152030,151990,152390,153580,152830,154360,153510,154460,151330,149770,155050,155470,156810,160670,161170,162040,162470,163970,165630,168900,173350,180890,179530,185190,189320,200650,201060,252820,353740,474910,356920,225350,284420,267090,283560,391390,373110)

# Linear regression
model <- lm(y ~ x)
coefs <- coef(model)

# Print regression coefficients
print(coefs)

# Estimating function
a <- 0.043
b <- 150000.0
fe <- function(x) {
  a*x + b
}

# Estimated values
xe <- seq(1, 10000000, by = 10000)
ye <- fe(xe)

png("data-drop-reg.png")
plot(x, y, main = "data.drop reg Linux")
abline(model, col = "blue")
lines(xe, ye, col = "magenta")
legend("bottomright", legend = sprintf("r = %.5f x + %.0f", coefs[2], coefs[1]), bty = "n")
legend("topleft", legend = sprintf("y = %.3f x + %.0f", a, b), bty = "n")

png("data-drop-log.png")
plot(x, y, log = "x", main = "data.drop log Linux")
lines(xe, ye, col = "magenta")
legend("topleft", legend = sprintf("y = %.3f x + %.0f", a, b), bty = "n")
