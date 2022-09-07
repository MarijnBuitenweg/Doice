use std::ops::Deref;

use itertools::Itertools;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::tup_swap;

/// Should be impld on anything that wants to be found with a certain name
pub trait Named {
    fn search_name(&self) -> &str;
}

/// This makes strings searchable by default
impl Named for String {
    fn search_name(&self) -> &str {
        self
    }
}

pub trait Search<'col, 'item> {
    const MAX_SCORE: f64 = 100_000.0;
    type SearchBy: 'item;
    type SearchFor: 'col;

    /// Finds the n entries that are the most similar to the provided item
    fn find_closest_matches(
        &'col self,
        item: Self::SearchBy,
        n: usize,
    ) -> Vec<(u32, Self::SearchFor)> {
        let mut distances = self.calculate_distances(item);
        // Sort it and take the n closest matches
        distances.sort_by_key(|(score, _)| *score);
        distances.into_iter().rev().take(n).collect()
    }

    /// Finds the entry that is the most similar to the provided item
    fn find_closest_match(&'col self, item: Self::SearchBy) -> Option<(u32, Self::SearchFor)> {
        let mut distances = self.calculate_distances(item);
        distances.sort_by_key(|(score, _)| *score);
        distances.into_iter().last()
    }

    /// Returns the index of the most similar item
    fn find_closest_match_index(&'col self, item: Self::SearchBy) -> Option<(u32, usize)> {
        Some(tup_swap(
            self.calculate_distances(item)
                .iter()
                .map(|(score, _)| score)
                .copied()
                .enumerate()
                .max_by_key(|(_, score)| *score)?,
        ))
    }

    /// Returns the indices of the n most similar items
    fn find_closest_matches_index(&'col self, item: Self::SearchBy, n: usize) -> Vec<(u32, usize)> {
        // Calculate the scores, and pair them with the indices
        let mut scores = self
            .calculate_distances(item)
            .iter()
            .map(|(score, _)| score)
            .copied()
            .enumerate()
            .collect_vec();

        // Sort results based on score
        scores.sort_by_key(|(_, score)| *score);
        // Take the n highest scores
        scores.into_iter().rev().take(n).map(tup_swap).collect()
    }

    /// Calculates a similarity score for each element that is searchable
    /// Scores calculated by this function should not exceed Self::MAX_SCORE
    /// Higher scores should correspond to more similar entries
    fn calculate_distances(&'col self, item: Self::SearchBy) -> Vec<(u32, Self::SearchFor)>;
}

impl<'col, 'item, T> Search<'col, 'item> for T
where
    'col: 'item,
    T: 'col + ?Sized,
    &'col T: IntoIterator,
    <&'col T as IntoIterator>::Item: Deref,
    <<&'col T as IntoIterator>::Item as Deref>::Target: Named,
{
    const MAX_SCORE: f64 = 100_000.0;

    type SearchBy = &'item str;

    type SearchFor = <&'col T as IntoIterator>::Item;

    /// This should be automatically implemented for any collection of named items that can be traversed in parallel
    /// It uses jaro similarity to calculate a score from 0 to Self::MAX_SCORE for each element in the collection
    fn calculate_distances(&'col self, name: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
        self.into_iter()
            .map(|item| {
                (
                    (strsim::jaro(item.search_name(), name) * Self::MAX_SCORE) as u32,
                    item,
                )
            })
            .collect()
    }
}

/// A trait that makes it so the implementor can be searched
/// The 'col lifetime is the lifetime of the implementor AKA collection.
/// The 'item lifetime is the lifetime of the thing that is being searched for
#[cfg(feature = "rayon")]
pub trait ParSearch<'col, 'item>: Search<'col, 'item, SearchFor: Send> + Send {
    /// Finds the n entries that are the most similar to the provided item
    fn par_find_closest_matches(
        &'col self,
        item: Self::SearchBy,
        n: usize,
    ) -> Vec<(u32, Self::SearchFor)> {
        let mut distances = self.par_calculate_distances(item);
        // Sort it and take the n closest matches
        distances.par_sort_by_key(|(score, _)| *score);
        distances.into_iter().rev().take(n).collect()
    }

    /// Finds the entry that is the most similar to the provided item
    fn par_find_closest_match(&'col self, item: Self::SearchBy) -> Option<(u32, Self::SearchFor)> {
        let mut distances = self.par_calculate_distances(item);
        distances.par_sort_by_key(|(score, _)| *score);
        distances.into_iter().last()
    }

    /// Returns the index of the most similar item
    fn par_find_closest_match_index(&'col self, item: Self::SearchBy) -> Option<(u32, usize)> {
        Some(tup_swap(
            self.par_calculate_distances(item)
                .iter()
                .map(|(score, _)| score)
                .copied()
                .enumerate()
                .max_by_key(|(_, score)| *score)?,
        ))
    }

    /// Returns the indices of the n most similar items
    fn par_find_closest_matches_index(
        &'col self,
        item: Self::SearchBy,
        n: usize,
    ) -> Vec<(u32, usize)> {
        // Calculate the scores, and pair them with the indices
        let mut scores = self
            .par_calculate_distances(item)
            .iter()
            .map(|(score, _)| score)
            .copied()
            .enumerate()
            .collect_vec();

        // Sort results based on score
        scores.par_sort_by_key(|(_, score)| *score);
        // Take the n highest scores
        scores.into_iter().rev().take(n).map(tup_swap).collect()
    }

    /// Calculates a similarity score for each element that is searchable
    /// Scores calculated by this function should not exceed Self::MAX_SCORE
    /// Higher scores should correspond to more similar entries
    fn par_calculate_distances(&'col self, item: Self::SearchBy) -> Vec<(u32, Self::SearchFor)>;
}

/// Blanked impl for any collection of named things that can be traversed in parallel
#[cfg(feature = "rayon")]
impl<'col, 'item, T> ParSearch<'col, 'item> for T
where
    T: 'col
        + Search<
            'col,
            'item,
            SearchBy = &'item str,
            SearchFor = <&'col T as IntoParallelIterator>::Item,
        >
        + ?Sized
        + Send,
    &'col T: IntoParallelIterator,
    <&'col T as IntoParallelIterator>::Item: Deref,
    <<&'col T as IntoParallelIterator>::Item as Deref>::Target: Named,
{
    /// This should be automatically implemented for any collection of named items that can be traversed in parallel
    /// It uses jaro similarity to calculate a score from 0 to Self::MAX_SCORE for each element in the collection
    fn par_calculate_distances(&'col self, name: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
        self.par_iter()
            .map(|item| {
                (
                    (strsim::jaro(item.search_name(), name) * Self::MAX_SCORE) as u32,
                    item,
                )
            })
            .collect()
    }
}

/// A trait that makes it so the implementor can be searched
/// The 'col lifetime is the lifetime of the implementor AKA collection.
/// The 'item lifetime is the lifetime of the thing that is being searched for
#[cfg(not(feature = "rayon"))]
pub trait ParSearch<'col, 'item>: Search<'col, 'item, SearchFor: Send> + Send {
    /// Finds the n entries that are the most similar to the provided item
    fn par_find_closest_matches(
        &'col self,
        item: Self::SearchBy,
        n: usize,
    ) -> Vec<(u32, Self::SearchFor)> {
        let mut distances = self.par_calculate_distances(item);
        // Sort it and take the n closest matches
        distances.sort_by_key(|(score, _)| *score);
        distances.into_iter().rev().take(n).collect()
    }

    /// Finds the entry that is the most similar to the provided item
    fn par_find_closest_match(&'col self, item: Self::SearchBy) -> Option<(u32, Self::SearchFor)> {
        let mut distances = self.par_calculate_distances(item);
        distances.sort_by_key(|(score, _)| *score);
        distances.into_iter().last()
    }

    /// Returns the index of the most similar item
    fn par_find_closest_match_index(&'col self, item: Self::SearchBy) -> Option<(u32, usize)> {
        Some(tup_swap(
            self.par_calculate_distances(item)
                .iter()
                .map(|(score, _)| score)
                .copied()
                .enumerate()
                .max_by_key(|(_, score)| *score)?,
        ))
    }

    /// Returns the indices of the n most similar items
    fn par_find_closest_matches_index(
        &'col self,
        item: Self::SearchBy,
        n: usize,
    ) -> Vec<(u32, usize)> {
        // Calculate the scores, and pair them with the indices
        let mut scores = self
            .par_calculate_distances(item)
            .iter()
            .map(|(score, _)| score)
            .copied()
            .enumerate()
            .collect_vec();

        // Sort results based on score
        scores.sort_by_key(|(_, score)| *score);
        // Take the n highest scores
        scores.into_iter().rev().take(n).map(tup_swap).collect()
    }

    /// Calculates a similarity score for each element that is searchable
    /// Scores calculated by this function should not exceed Self::MAX_SCORE
    /// Higher scores should correspond to more similar entries
    fn par_calculate_distances(&'col self, item: Self::SearchBy) -> Vec<(u32, Self::SearchFor)>;
}

/// Blanked impl for any collection of named things that can be traversed in parallel
#[cfg(not(feature = "rayon"))]
impl<'col, 'item, T> ParSearch<'col, 'item> for T
where
    T: 'col
        + Search<
            'col,
            'item,
            SearchBy = &'item str,
            SearchFor = <&'col T as IntoIterator>::Item,
        >
        + ?Sized
        + Send,
    &'col T: IntoIterator,
    <&'col T as IntoIterator>::Item: Deref + Send,
    <<&'col T as IntoIterator>::Item as Deref>::Target: Named,
{
    /// This should be automatically implemented for any collection of named items that can be traversed in parallel
    /// It uses jaro similarity to calculate a score from 0 to Self::MAX_SCORE for each element in the collection
    fn par_calculate_distances(&'col self, name: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
        self.into_iter()
            .map(|item| {
                (
                    (strsim::jaro(item.search_name(), name) * Self::MAX_SCORE) as u32,
                    item,
                )
            })
            .collect()
    }
}
