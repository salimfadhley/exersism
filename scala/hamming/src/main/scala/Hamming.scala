object Hamming {
  def distance(a:String, b:String):Option[Integer] = {
    (a,b) match {
      case (aa,bb) if aa.length == bb.length => Some(a.toIterable.zip(b.toIterable).count((x: (Char, Char)) =>x._1 != x._2))
      case _ => None
    }
  }
}