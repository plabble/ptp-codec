use binary_codec::{BinaryDeserializer, BinarySerializer};
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Plabble DateTime since epoch (01-01-2025T00:00:00Z)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlabbleDateTime(pub DateTime<Utc>);

fn epoch() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()
}

impl PlabbleDateTime {
    pub fn timestamp(&self) -> i64 {
        (self.0 - epoch()).num_seconds()
    }

    pub fn new(timestamp: i64) -> Self {
        Self(epoch() + Duration::seconds(timestamp))
    }
}

impl<T: Clone> BinarySerializer<T> for PlabbleDateTime {
    fn write_bytes(
        &self,
        stream: &mut binary_codec::BitStreamWriter,
        _: Option<&mut binary_codec::SerializerConfig<T>>,
    ) -> Result<(), binary_codec::SerializationError> {
        stream.write_fixed_int(self.timestamp() as u32);
        Ok(())
    }
}

impl<T: Clone> BinaryDeserializer<T> for PlabbleDateTime {
    fn read_bytes(
        stream: &mut binary_codec::BitStreamReader,
        _: Option<&mut binary_codec::SerializerConfig<T>>,
    ) -> Result<Self, binary_codec::DeserializationError> {
        let seconds: u32 = stream.read_fixed_int()?;
        Ok(PlabbleDateTime::new(seconds as i64))
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use binary_codec::{BinaryDeserializer, BinarySerializer};
    use chrono::{TimeZone, Utc};

    use super::PlabbleDateTime;

    #[test]
    fn can_convert_to_seconds_from_epoch_and_back() {
        let date = Utc.with_ymd_and_hms(2025, 5, 25, 12, 30, 0).unwrap();
        let expected_seconds = 12_486_600u32;
        let expected_bytes = u32::to_be_bytes(expected_seconds);

        let bytes = BinarySerializer::<()>::to_bytes(&PlabbleDateTime(date), None).unwrap();
        assert_eq!(bytes, expected_bytes);
        assert_eq!(bytes, vec![0, 190, 135, 200]);

        let deserialized: PlabbleDateTime =
            BinaryDeserializer::<()>::from_bytes(&bytes, None).unwrap();
        assert_eq!(deserialized.0, date);
    }

    #[test]
    fn plabble_datetime_max_value() {
        let max_bytes = u32::MAX.to_be_bytes();
        let max_date: PlabbleDateTime =
            BinaryDeserializer::<()>::from_bytes(&max_bytes, None).unwrap();

        assert_eq!("2161-02-07T06:28:15+00:00", max_date.0.to_rfc3339());
    }
}
