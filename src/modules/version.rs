use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u8> = s
            .split('.')
            .map(|c| c.parse::<u8>())
            .collect::<Result<_, _>>()
            .map_err(|e| anyhow!("Invalid version format: {e}"))?;

        match values.as_slice() {
            &[major, minor, patch] => Ok(Version {
                major,
                minor,
                patch,
            }),
            _ => Err(anyhow!("Invalid version format")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() -> anyhow::Result<()> {
        let s = "0.0.1";
        let v: Version = s.parse()?;
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 1);

        let s = "0.1.0";
        let v: Version = s.parse()?;
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 1);
        assert_eq!(v.patch, 0);

        let s = "1.0.0";
        let v: Version = s.parse()?;
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);

        let s = "1.1.1";
        let v: Version = s.parse()?;
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 1);
        assert_eq!(v.patch, 1);

        let s = "not a version";
        let v: Result<Version, _> = s.parse();
        assert!(v.is_err());

        let s = "not.a.version";
        let v: Result<Version, _> = s.parse();
        assert!(v.is_err());

        let s = "0.0";
        let v: Result<Version, _> = s.parse();
        assert!(v.is_err());

        Ok(())
    }
}
