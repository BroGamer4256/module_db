use itertools::Itertools;
use serde::de::{MapAccess, Visitor};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::collections::BTreeMap;

#[derive(Deserialize, Clone)]
pub struct Module {
    pub chara: crate::Chara,
    pub cos: String,
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct Costume {
    pub id: i32,
    pub item: Vec<i32>,
}

#[derive(Deserialize, Clone)]
pub struct CostumeItem {
    pub no: i32,
    pub objset: Vec<String>,
    pub sub_id: i32,
}

#[derive(Deserialize, Clone)]
pub struct CstmItem {
    pub bind_module: Option<i32>,
    pub chara: crate::Chara,
    pub id: i32,
    pub name: String,
    pub parts: crate::ItemPart,
}

#[derive(Clone)]
pub struct DivaTbl<T> {
    pub data: Vec<T>,
}

struct DivaMapVisitor<T> {
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> DivaMapVisitor<T> {
    fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for DivaMapVisitor<T> {
    type Value = DivaTbl<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("module")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut vec = Vec::new();
        loop {
            let entry = access.next_entry::<i32, T>();
            let Ok(entry) = entry else {
                continue;
            };
            let Some((_, value)) = entry else {
                break;
            };
            vec.push(value);
        }

        Ok(Self::Value { data: vec })
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for DivaTbl<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(DivaMapVisitor::new())
    }
}

pub fn clean_input(input: &str) -> String {
    input
        .lines()
        .dedup()
        .filter(|line| line.contains('='))
        .collect::<Vec<_>>()
        .join("\n")
}

impl Module {
    pub async fn parse<P: AsRef<std::path::Path>>(path: P) -> Option<DivaTbl<Self>> {
        let path = path.as_ref();
        if !path.exists() {
            return None;
        }

        let str = path.to_str()?;
        let contents = if str.ends_with("gm_module_tbl.farc") {
            let farc = farc::Farc::from_file(path).ok()?;
            let file = farc.entries.get("gm_module_id.bin")?;
            let buf = file.to_buf_const()?;
            String::from_utf8(buf.to_vec()).ok()?
        } else if str.ends_with("gm_module_id.bin") {
            tokio::fs::read_to_string(path).await.ok()?
        } else {
            return None;
        };

        serde_divatree::from_str(&clean_input(&contents)).ok()
    }
}

impl Costume {
    pub async fn parse<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Option<BTreeMap<crate::Chara, DivaTbl<Self>>> {
        let path = path.as_ref();
        if !path.exists() {
            return None;
        }
        if !path.to_str()?.ends_with("chritm_prop.farc") {
            return None;
        }

        let mut map = BTreeMap::new();
        let farc = farc::Farc::from_file(path).ok()?;
        for (name, data) in farc.entries {
            if !name.ends_with("itm_tbl.txt") {
                continue;
            }
            let chara = match name.trim_end_matches("itm_tbl.txt") {
                "mik" => crate::Chara::Miku,
                "rin" => crate::Chara::Rin,
                "len" => crate::Chara::Len,
                "luk" => crate::Chara::Luka,
                "ner" => crate::Chara::Neru,
                "hak" => crate::Chara::Haku,
                "kai" => crate::Chara::Kaito,
                "mei" => crate::Chara::Meiko,
                "sak" => crate::Chara::Sakine,
                "tet" => crate::Chara::Teto,
                "ext" => crate::Chara::Extra,
                _ => continue,
            };
            let buf = data.to_buf_const()?;
            let data = String::from_utf8(buf.to_vec()).ok()?;
            let data = clean_input(&data);
            let data = data
                .lines()
                .filter(|line| line.starts_with("cos."))
                .collect::<Vec<_>>()
                .join("\n");
            let data = serde_divatree::from_str(&data).ok()?;
            map.insert(chara, data);
        }

        if map.len() == 0 {
            None
        } else {
            Some(map)
        }
    }
}

impl CostumeItem {
    pub async fn parse<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Option<BTreeMap<crate::Chara, DivaTbl<Self>>> {
        let path = path.as_ref();
        if !path.exists() {
            return None;
        }
        if !path.to_str()?.ends_with("chritm_prop.farc") {
            return None;
        }

        let mut map = BTreeMap::new();
        let farc = farc::Farc::from_file(path).ok()?;
        for (name, data) in farc.entries {
            if !name.ends_with("itm_tbl.txt") {
                continue;
            }
            let chara = match name.trim_end_matches("itm_tbl.txt") {
                "mik" => crate::Chara::Miku,
                "rin" => crate::Chara::Rin,
                "len" => crate::Chara::Len,
                "luk" => crate::Chara::Luka,
                "ner" => crate::Chara::Neru,
                "hak" => crate::Chara::Haku,
                "kai" => crate::Chara::Kaito,
                "mei" => crate::Chara::Meiko,
                "sak" => crate::Chara::Sakine,
                "tet" => crate::Chara::Teto,
                "ext" => crate::Chara::Extra,
                _ => continue,
            };
            let buf = data.to_buf_const()?;
            let data = String::from_utf8(buf.to_vec()).ok()?;
            let data = clean_input(&data);
            let data = data
                .lines()
                .filter(|line| line.starts_with("item."))
                .collect::<Vec<_>>()
                .join("\n");
            let data = serde_divatree::from_str(&data).ok()?;
            map.insert(chara, data);
        }

        if map.len() == 0 {
            None
        } else {
            Some(map)
        }
    }
}

impl TryInto<crate::CostumeItem> for CostumeItem {
    type Error = String;

    fn try_into(self) -> Result<crate::CostumeItem, Self::Error> {
        let sub = self.sub_id.try_into()?;

        Ok(crate::CostumeItem {
            id: self.no,
            objset: self.objset,
            sub,
        })
    }
}

impl CstmItem {
    pub async fn parse<P: AsRef<std::path::Path>>(path: P) -> Option<DivaTbl<Self>> {
        let path = path.as_ref();
        if !path.exists() {
            return None;
        }

        let str = path.to_str()?;
        let contents = if str.ends_with("gm_customize_item_tbl.farc") {
            let farc = farc::Farc::from_file(path).ok()?;
            let file = farc.entries.get("gm_customize_item_id.bin")?;
            let buf = file.to_buf_const()?;
            String::from_utf8(buf.to_vec()).ok()?
        } else if str.ends_with("gm_customize_item_id.bin") {
            tokio::fs::read_to_string(path).await.ok()?
        } else {
            return None;
        };

        serde_divatree::from_str(&clean_input(&contents)).ok()
    }
}

#[derive(Deserialize, Clone)]
pub struct ModStringArray {
    #[serde(flatten)]
    pub data: Option<ModStringArrayData>,
    pub en: Option<ModStringArrayData>,
    pub cn: Option<ModStringArrayData>,
    pub fr: Option<ModStringArrayData>,
    pub ge: Option<ModStringArrayData>,
    pub it: Option<ModStringArrayData>,
    pub kr: Option<ModStringArrayData>,
    pub sp: Option<ModStringArrayData>,
    pub tw: Option<ModStringArrayData>,
}

#[serde_as]
#[derive(Deserialize, Clone)]
pub struct ModStringArrayData {
    #[serde_as(as = "Option<BTreeMap<DisplayFromStr, _>>")]
    pub module: Option<BTreeMap<i32, String>>,
    #[serde(alias = "cstm_item")]
    #[serde_as(as = "Option<BTreeMap<DisplayFromStr, _>>")]
    pub customize: Option<BTreeMap<i32, String>>,
}

impl ModStringArray {
    pub async fn parse<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return None;
        }

        let contents = tokio::fs::read_to_string(path).await.ok()?;
        toml::from_str(&contents).ok()
    }
}
