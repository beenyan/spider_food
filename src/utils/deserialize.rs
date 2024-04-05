use serde::de;

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let num: u8 = de::Deserialize::deserialize(deserializer).unwrap_or(0);

    match num {
        0 => Ok(false),
        _ => Ok(true),
    }
}

pub fn str_to_number<'de, D>(deserializer: D) -> Result<isize, D::Error>
where
    D: de::Deserializer<'de>,
{
    let num_str: &str = de::Deserialize::deserialize(deserializer).unwrap_or("-1");

    Ok(num_str.parse::<isize>().unwrap_or(-1))
}

pub fn bool_not<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let bl: bool = de::Deserialize::deserialize(deserializer).unwrap();

    Ok(!bl)
}
