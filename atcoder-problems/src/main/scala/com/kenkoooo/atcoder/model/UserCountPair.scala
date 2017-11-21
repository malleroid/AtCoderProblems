package com.kenkoooo.atcoder.model

import com.kenkoooo.atcoder.common.TypeAnnotations.UserId
import com.kenkoooo.atcoder.db.SQLInsertSelectSupport
import scalikejdbc.{ParameterBinder, WrappedResultSet}

trait UserCountPair {
  def userId: UserId

  def problemCount: Int
}

trait UserCountPairSupport[T <: UserCountPair, P] extends SQLInsertSelectSupport[T] {
  override def apply(resultName: scalikejdbc.ResultName[T])(rs: WrappedResultSet): T = {
    apply(userId = rs.string(resultName.userId), problemCount = rs.int(resultName.problemCount))
  }

  override def createMapping(seq: Seq[T]): Seq[Seq[(scalikejdbc.SQLSyntax, ParameterBinder)]] = {
    val column = this.column
    seq.map { t =>
      Seq(column.userId -> t.userId, column.problemCount -> t.problemCount)
    }
  }

  def apply(userId: UserId, problemCount: Int): T

  def parentSupport: SQLInsertSelectSupport[P]
}

case class ShortestSubmissionCount(userId: UserId, problemCount: Int) extends UserCountPair

object ShortestSubmissionCount extends UserCountPairSupport[ShortestSubmissionCount, Shortest] {
  override protected def definedTableName = "shortest_submission_count"

  override def parentSupport: Shortest.type = Shortest
}