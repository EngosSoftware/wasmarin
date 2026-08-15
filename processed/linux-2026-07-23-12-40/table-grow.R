x <- seq(1, 10000000, length.out = 50)
y <- seq(1, 1000000000, length.out = 50)

z <- outer(x, y, function(x, y) {
  4383*x + 3471*y + 108145
})

png("table-grow.png")
persp(
  x, y, z,
  theta = 40,
  phi = 10,
  col = "cyan",
  xlab = "initial",
  ylab = "grow",
  zlab = "gas"
)
