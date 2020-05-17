object Anagram {

  def count(w: String): Map[Char, Int] = {
    w.groupBy(identity).mapValues(_.length)
  }

  def isAnagram(word: String, w: String): Boolean = {
    w match {
      case ww if ww == word => false
      case x => count(x) == count(word)
    }
  }

  def findAnagrams(word: String, candidates: Seq[String]): Seq[String] = {
    candidates.filter((w: String) => this.isAnagram(word.toLowerCase(), w.toLowerCase()))
  }
}