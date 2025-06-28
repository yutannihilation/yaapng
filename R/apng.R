#' Generate APNG file from PNG files
#'
#' @param input_files paths to input PNG files
#' @param output_file path to output APNG file
#' @param delay_num the numerator of the frame delay (delay is \eqn{\frac{delay_num}{delay_den}})
#' @param delay_den the denominator of the frame delay (delay is \eqn{\frac{delay_num}{delay_den}})
#' @export
apng <- function(
  input_files,
  output_file = "output.png",
  delay_num = 0L,
  delay_den = 100L
) {
  apng_inner(
    input_files,
    output_file,
    as.integer(delay_num),
    as.integer(delay_den)
  )
}
