x <- seq(1, 10000000, by = 100000)
y <- seq(1, 1000000000, by = 1000000)

z <- outer(x, y, function(x, y) {
  4383*x + 3471*y + 108145
})

png("table-grow.png")
persp(
  x, y, z,
  theta = 30,
  phi = 25,
  col = "lightblue",
  xlab = "x",
  ylab = "y",
  zlab = "z"
)
