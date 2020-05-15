case class Matrix(rows: Vector[Vector[Int]]) {
  def row(r: Int): Vector[Int] = rows(r)

  def column(c: Int): Vector[Int] = rows.map((r: Seq[Int]) => r(c))
}

object Matrix {
  def apply(input_data: String): Matrix = {
    val rows: Vector[Vector[Int]] = input_data.split('\n').toVector.map((r: String) => r.split(' ').toVector.map((str: String) => str.toInt))
    Matrix(rows)
  }
}