
<!-- README.md is generated from README.Rmd. Please edit that file -->

# yaapng

<!-- badges: start -->

<!-- badges: end -->

**Y**et **A**nather R Package for **APNG**.

## Installation

You can install the development version of yaapng from
[GitHub](https://github.com/) with:

``` r
# install.packages("pak")
pak::pak("yutannihilation/yaapng")
```

## Example

``` r
library(yaapng)

unlink("anim", recursive = TRUE, force = TRUE)
dir.create("anim", showWarnings = FALSE)

x <- 1:1000
frame_count <- 1000

for (i in 1:frame_count) {
  png(filename = file.path("anim", paste(i, ".png")))
  plot(
    x,
    sin((i / x) %% 1) * (-1)^x,
    ylim = c(-1, 1),
    col = ifelse(x == i, "red", "black"),
    pch = ifelse(x == i, max((-1)^i, 0) * 5, ifelse(x > i, 8, 1))
  )
  dev.off()
}

apng(file.path("anim", paste(1:frame_count, ".png")), "output.png")
```

``` r
knitr::include_graphics("output.png")
```

<img src="output.png" width="100%" />
