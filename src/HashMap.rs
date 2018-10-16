use std::rc::Rc;
pub struct HashMap<K, V> {
    pub table: Vec<MapEntry<K, V>>,
    pub capacity: i32
}

impl <K, V> HashMap<K, V> {

}
pub struct MapEntry<K, V> {
    pub key: K,
    pub value: V,
    pub hash: i32,
    pub next: Rc<MapEntry<K, V>>
}