import java.time.{DayOfWeek, LocalDate}

import Schedule.Schedule

case class Meetup(month: Int, year: Int) {

  def day(dayOfWeek: Int, schedule: Schedule): LocalDate = {
    val firstDay = LocalDate.of(year, month, 1)
    val candidate_days = (0 to 32).map(firstDay.plusDays(_)).filter(_.getMonthValue == month).filter(dayOfWeek == _.getDayOfWeek.getValue)

    schedule match {
      case Schedule.Teenth => candidate_days.filter((date: LocalDate) => 13 <= date.getDayOfMonth && date.getDayOfMonth <= 19).head
      case Schedule.First => candidate_days.head
      case Schedule.Second => candidate_days(1)
      case Schedule.Third => candidate_days(2)
      case Schedule.Fourth => candidate_days(3)
      case Schedule.Last => candidate_days.last
    }

  }
}

object Schedule extends Enumeration {
  type Schedule = Value
  val Teenth, First, Second, Third, Fourth, Last = Value
}

object Meetup {
  val Mon: Int = DayOfWeek.MONDAY.getValue
  val Tue: Int = DayOfWeek.TUESDAY.getValue
  val Wed: Int = DayOfWeek.WEDNESDAY.getValue
  val Thu: Int = DayOfWeek.THURSDAY.getValue
  val Fri: Int = DayOfWeek.FRIDAY.getValue
  val Sat: Int = DayOfWeek.SATURDAY.getValue
  val Sun: Int = DayOfWeek.SUNDAY.getValue
}
