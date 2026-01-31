pub trait LabelResolverTrait {
    fn lookup(&mut self, label: &str) -> i64;
}
