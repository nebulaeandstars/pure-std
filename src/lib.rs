pub mod prelude {
    pub trait Sort {
        type Item;

        fn sorted(self) -> Self;
        fn sorted_unstable(self) -> Self;

        fn sorted_by<F>(self, compare: F) -> Self
        where
            F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering;
        fn sorted_unstable_by<F>(self, compare: F) -> Self
        where
            F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering;

        fn sorted_by_key<K, F>(self, compare: F) -> Self
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord;
        fn sorted_unstable_by_key<K, F>(self, compare: F) -> Self
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord;

        fn sorted_by_cached_key<K, F>(self, compare: F) -> Self
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord;
    }

    impl<T: Ord> Sort for Vec<T> {
        type Item = T;

        fn sorted(mut self) -> Self {
            self.sort();
            self
        }
        fn sorted_unstable(mut self) -> Self {
            self.sort_unstable();
            self
        }

        fn sorted_by<F>(mut self, compare: F) -> Self
        where
            F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
        {
            self.sort_by(compare);
            self
        }
        fn sorted_unstable_by<F>(mut self, compare: F) -> Self
        where
            F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
        {
            self.sort_unstable_by(compare);
            self
        }

        fn sorted_by_key<K, F>(mut self, compare: F) -> Self
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord,
        {
            self.sort_by_key(compare);
            self
        }
        fn sorted_unstable_by_key<K, F>(mut self, compare: F) -> Self
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord,
        {
            self.sort_unstable_by_key(compare);
            self
        }

        fn sorted_by_cached_key<K, F>(mut self, compare: F) -> Self
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord,
        {
            self.sort_by_cached_key(compare);
            self
        }
    }
}
