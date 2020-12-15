use crate::song::hash::{HashMd5, HashSha256};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Converter {
    pub md5_to_sha256: HashMap<HashMd5, HashSha256>,
    pub sha256_to_md5: HashMap<HashSha256, HashMd5>,
}

impl Converter {
    pub fn new(
        md5_to_sha256: HashMap<HashMd5, HashSha256>,
        sha256_to_md5: HashMap<HashSha256, HashMd5>,
    ) -> Converter {
        Converter {
            md5_to_sha256,
            sha256_to_md5,
        }
    }

    pub fn get_md5(&self, sha256: &HashSha256) -> Option<HashMd5> {
        self.sha256_to_md5.get(&sha256).cloned()
    }

    pub fn get_sha256(&self, md5: &HashMd5) -> Option<HashSha256> {
        self.md5_to_sha256.get(&md5).cloned()
    }
}

#[cfg(test)]
mod test {
    use crate::song::hash::{HashMd5, HashSha256};
    use crate::song::hash_converter::Converter;
    use std::collections::HashMap;

    #[test]
    fn get() {
        let mut builder = Builder::new();

        let m1: HashMd5 = "m1".parse().unwrap();
        let m2: HashMd5 = "m2".parse().unwrap();
        let m3: HashMd5 = "m3".parse().unwrap();
        let s1: HashSha256 = "s1".parse().unwrap();
        let s2: HashSha256 = "s2".parse().unwrap();
        let s3: HashSha256 = "s3".parse().unwrap();

        builder.push(m1.clone(), s1.clone());
        builder.push(m2.clone(), s2.clone());

        let obj = Builder::build(builder);

        assert_eq!(obj.get_md5(&s1), Some(m1.clone()));
        assert_eq!(obj.get_sha256(&m1), Some(s1.clone()));

        assert_eq!(obj.get_md5(&s3), None);
        assert_eq!(obj.get_sha256(&m3), None);
    }

    struct Builder {
        md5_to_sha256: HashMap<HashMd5, HashSha256>,
        sha256_to_md5: HashMap<HashSha256, HashMd5>,
    }

    impl Builder {
        pub fn new() -> Builder {
            Builder {
                md5_to_sha256: HashMap::new(),
                sha256_to_md5: HashMap::new(),
            }
        }

        fn push(&mut self, md5: HashMd5, sha256: HashSha256) {
            self.sha256_to_md5.insert(sha256.clone(), md5.clone());
            self.md5_to_sha256.insert(md5, sha256);
        }

        fn build(builder: Self) -> Converter {
            Converter::new(builder.md5_to_sha256, builder.sha256_to_md5)
        }
    }
}
