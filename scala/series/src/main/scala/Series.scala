object Series {
  def slices(slice_length: Int, string: String): List[List[Int]] = {
    val ints: Seq[Int] = string.toList.map((c: Char) =>c.toInt-48)
    (0 to (string.length - slice_length)).map((i: Int) =>ints.slice(i, i+slice_length).toList).toList
  }
}