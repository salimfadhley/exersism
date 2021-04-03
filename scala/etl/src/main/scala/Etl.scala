object Etl {

  def transform(inp:Map[Int,Seq[String]]):Map[String,Int] = {
    inp.flatMap(x => x._2.map(y=>(y.map(_.toLower),x._1))).toMap
  }
}
