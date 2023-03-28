use chrono::{Duration, NaiveDateTime};
use supermemo2::Item;

use super::data_scaffold::learn_word::{LearnWord, LearnWordQueryBuilder};
use crate::errors;

use super::DbConnection;

impl DbConnection {
    pub async fn get_review_words(&self, dt: String) -> Result<Vec<LearnWord>, errors::Error> {
        let date = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S")?;

        let sql = format!(
            "SELECT * FROM learn_word WHERE next_learn_at like '{} %'",
            date.format("%Y-%m-%d").to_string()
        );

        let words: Vec<LearnWord> = sqlx::query_as(&sql).fetch_all(&self.conn).await?;

        Ok(words)
    }

    pub async fn add_review_word(&self, word: String) -> Result<(), errors::Error> {
        let brief_word = self.get_brief_word_info(None, Some(word.clone())).await?;

        if brief_word.is_empty() {
            return Err(errors::Error::WordNotFound(word));
        }

        let after_10_minutes = chrono::Utc::now().naive_utc() + Duration::minutes(10);
        let after_10_minutes = after_10_minutes.format("%Y-%m-%d %H:%M:%S").to_string();

        let sql = "INSERT INTO learn_word (word, paraphrase, next_learn_at) VALUES (?, ?, ?)";

        sqlx::query(&sql)
            .bind(word)
            .bind(brief_word[0].paraphrase.clone())
            .bind(after_10_minutes)
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    // status: 0: 忘了, 1: 看答案了, 4: 记住了
    pub async fn learn_word(
        &self,
        word_id: i64,
        learn_count: i64,
        next_learn_at: String,
        status: u8,
    ) -> Result<(), errors::Error> {
        let q = LearnWordQueryBuilder::default().id(Some(word_id)).build()?;

        let (next_time, learn_count) = get_next_time(status, learn_count, next_learn_at.clone())?;

        let sql = format!(
            "UPDATE learn_word SET next_learn_at = ?, learn_count = ? {}",
            q.to_sql_condition()
        );

        sqlx::query(&sql)
            .bind(next_time)
            .bind(learn_count)
            .execute(&self.conn)
            .await?;

        Ok(())
    }
}

/*
   match learn_count {
       0 => 0
       1 => 1,
       2 => 6,
       _ => (6.0 * 1.5f64.powf(learn_count as f64 - 2.0)).ceil() as i64,
   }
*/
fn get_next_time(
    status: u8,
    learn_count: i64,
    this_time: String,
) -> Result<(String, i64), errors::Error> {
    // 返回下次学习时间和学习次数
    let next_learn_at = NaiveDateTime::parse_from_str(&this_time, "%Y-%m-%d %H:%M:%S").unwrap();

    let sm2 = Item::new(learn_count as usize, 1.5);
    let sm2 = sm2.review(status)?;
    let interval = sm2.interval();
    let learn_count = sm2.repetitions();

    Ok((
        deal_time(interval as i64, next_learn_at),
        learn_count as i64,
    ))
}

fn deal_time(interval: i64, now: NaiveDateTime) -> String {
    let next_time = now + Duration::days(interval);
    next_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_time() {
        let next_time = get_next_time(0, 1, "2021-01-01 00:00:00".to_string()).unwrap();
        assert_eq!(next_time, ("2021-01-02 00:00:00".to_string(), 1));
    }

    #[test]
    fn test_deal_time() {
        let now =
            NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next_time = deal_time(1, now);
        assert_eq!(next_time, "2021-01-02 00:00:00");
    }
}
