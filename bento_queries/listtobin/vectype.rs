#[derive(Debug, Clone)]
pub enum VecType {
    I8Vec(Vec<i8>),
    U8Vec(Vec<u8>),
    I32Vec(Vec<i32>),
    U32Vec(Vec<u32>),
    I64Vec(Vec<i64>),
    U64Vec(Vec<u64>),
    StringVec(Vec<String>),
}

impl VecType {
    pub fn iter(&self) -> VecTypeIter {
        match self {
            VecType::I8Vec(v) => VecTypeIter::I8Iter(v.iter()),
            VecType::U8Vec(v) => VecTypeIter::U8Iter(v.iter()),
            VecType::I32Vec(v) => VecTypeIter::I32Iter(v.iter()),
            VecType::U32Vec(v) => VecTypeIter::U32Iter(v.iter()),
            VecType::I64Vec(v) => VecTypeIter::I64Iter(v.iter()),
            VecType::U64Vec(v) => VecTypeIter::U64Iter(v.iter()),
            VecType::StringVec(v) => VecTypeIter::StringIter(v.iter()),
        }
    }

    pub fn into_vec_f64(self) -> Vec<f64> {
        match self {
            VecType::I8Vec(v) => v.into_iter().map(|x| x as f64).collect(),
            VecType::U8Vec(v) => v.into_iter().map(|x| x as f64).collect(),
            VecType::I32Vec(v) => v.into_iter().map(|x| x as f64).collect(),
            VecType::U32Vec(v) => v.into_iter().map(|x| x as f64).collect(),
            VecType::I64Vec(v) => v.into_iter().map(|x| x as f64).collect(),
            VecType::U64Vec(v) => v.into_iter().map(|x| x as f64).collect(),
            VecType::StringVec(v) => v
                .into_iter()
                .map(|s| s.parse::<f64>().unwrap_or(0.0))
                .collect(),
        }
    }
}

// Enum to handle the iterator for each VecType variant
pub enum VecTypeIter<'a> {
    I8Iter(std::slice::Iter<'a, i8>),
    U8Iter(std::slice::Iter<'a, u8>),
    I32Iter(std::slice::Iter<'a, i32>),
    U32Iter(std::slice::Iter<'a, u32>),
    I64Iter(std::slice::Iter<'a, i64>),
    U64Iter(std::slice::Iter<'a, u64>),
    StringIter(std::slice::Iter<'a, String>),
}

// Implement Iterator for VecTypeIter
impl<'a> Iterator for VecTypeIter<'a> {
    type Item = VecTypeItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            VecTypeIter::I8Iter(iter) => iter.next().map(VecTypeItem::I8Item),
            VecTypeIter::U8Iter(iter) => iter.next().map(VecTypeItem::U8Item),
            VecTypeIter::I32Iter(iter) => iter.next().map(VecTypeItem::I32Item),
            VecTypeIter::U32Iter(iter) => iter.next().map(VecTypeItem::U32Item),
            VecTypeIter::I64Iter(iter) => iter.next().map(VecTypeItem::I64Item),
            VecTypeIter::U64Iter(iter) => iter.next().map(VecTypeItem::U64Item),
            VecTypeIter::StringIter(iter) => iter.next().map(VecTypeItem::StringItem),
        }
    }
}

// Enum for item type returned by the iterator
#[derive(Debug, Clone, Copy)]
pub enum VecTypeItem<'a> {
    I8Item(&'a i8),
    U8Item(&'a u8),
    I32Item(&'a i32),
    U32Item(&'a u32),
    I64Item(&'a i64),
    U64Item(&'a u64),
    StringItem(&'a String),
}

// Function to extract values from VecTypeItem
pub fn extract_i8_from_vec_type_item(item: VecTypeItem) -> Option<i8> {
    match item {
        VecTypeItem::I8Item(value) => Some(*value),
        _ => None,
    }
}

pub fn extract_u8_from_vec_type_item(item: VecTypeItem) -> Option<u8> {
    match item {
        VecTypeItem::U8Item(value) => Some(*value),
        _ => None,
    }
}

pub fn extract_i32_from_vec_type_item(item: VecTypeItem) -> Option<i32> {
    match item {
        VecTypeItem::I32Item(value) => Some(*value),
        _ => None,
    }
}

pub fn extract_u32_from_vec_type_item(item: VecTypeItem) -> Option<u32> {
    match item {
        VecTypeItem::U32Item(value) => Some(*value),
        _ => None,
    }
}

pub fn extract_i64_from_vec_type_item(item: VecTypeItem) -> Option<i64> {
    match item {
        VecTypeItem::I64Item(value) => Some(*value),
        _ => None,
    }
}

pub fn extract_u64_from_vec_type_item(item: VecTypeItem) -> Option<u64> {
    match item {
        VecTypeItem::U64Item(value) => Some(*value),
        _ => None,
    }
}

pub fn extract_string_from_vec_type_item(item: VecTypeItem) -> Option<String> {
    match item {
        VecTypeItem::StringItem(value) => Some(value.clone()),
        _ => None,
    }
}
