use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::NaiveDate;
use serde::{self, de, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct LotteryTicket {
    #[serde(rename = "Draw date")]
    #[serde(deserialize_with = "deserialize_date_string")]
    pub date: NaiveDate,
    #[serde(rename = "Winning Numbers")]
    #[serde(deserialize_with = "deserialize_numbers_string")]
    pub numbers: Vec<i8>,
}

fn deserialize_numbers_string<'de, D>(deserializer: D) -> Result<Vec<i8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> de::Visitor<'de> for StringVisitor {
        type Value = Vec<i8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing number data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut out = Vec::new();
            let iter = v.split(" - ");
            let iter_len = iter.clone().collect::<Vec<&str>>().len();
            for item in iter {
                out.push(item.parse::<i8>().unwrap())
            }
            if !out.is_empty() {
                return Ok(out);
            } else if iter_len != out.len() {
                return Err(de::Error::custom("Fuck"));
            } else {
                return Err(de::Error::custom("Fuck"));
            }
        }
    }

    deserializer.deserialize_any(StringVisitor)
}

fn deserialize_date_string<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> de::Visitor<'de> for StringVisitor {
        type Value = NaiveDate;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing date data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let iter = v.split(", ");
            let date_vec = iter.last().unwrap().split("/").collect::<Vec<&str>>();

            // YIKES
            #[allow(unused_assignments)]
            let mut year = date_vec[2].parse::<i32>().unwrap();
            if date_vec[2].parse::<i32>().unwrap() >= 90 {
                year = format!("19{}", date_vec[2]).parse::<i32>().unwrap();
            } else {
                year = format!("20{}", date_vec[2]).parse::<i32>().unwrap();
            }

            Ok(NaiveDate::from_ymd(
                year,
                date_vec[0].parse::<u32>().unwrap(),
                date_vec[1].parse::<u32>().unwrap(),
            ))
        }
    }

    deserializer.deserialize_any(StringVisitor)
}

pub fn data_keymap() -> Option<Vec<LotteryTicket>> {
    let path = "data.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).ok()?;
    let range = workbook
        .worksheet_range("Drawing History")
        .ok_or(Error::Msg("Cannot find 'Drawing History'"))
        .ok()?
        .ok()?;

    // let mut iter = RangeDeserializerBuilder::new().from_range(&range).ok()?;
    let iter = RangeDeserializerBuilder::new()
        .from_range::<_, LotteryTicket>(&range)
        .ok()?;

    let mut output = Vec::new();

    for line in iter {
        if let Ok(line) = line {
            output.push(line);
        }
    }

    if !output.is_empty() {
        Some(output.into_iter().rev().collect::<Vec<LotteryTicket>>())
    } else {
        None
    }
}
